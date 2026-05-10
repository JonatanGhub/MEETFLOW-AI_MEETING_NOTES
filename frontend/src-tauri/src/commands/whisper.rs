use std::path::PathBuf;
use std::sync::Arc;

use tauri::{AppHandle, Emitter, State};

use crate::db::DbPool;
use crate::error::MeetflowError;
use crate::storage;
use crate::whisper::{download, engine::WhisperEngine, ModelCatalogEntry, MODEL_CATALOG};

/// List all models in the catalog with their download status.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelStatus {
    #[serde(flatten)]
    pub entry: &'static ModelCatalogEntry,
    pub downloaded: bool,
    pub path: Option<String>,
}

#[tauri::command]
pub fn list_whisper_models(app: AppHandle) -> Result<Vec<ModelStatus>, MeetflowError> {
    let models_dir = storage::models_dir(&app)?;
    let statuses = MODEL_CATALOG
        .iter()
        .map(|e| {
            let path = models_dir.join(format!("ggml-{}.bin", e.id));
            let downloaded = path.exists();
            ModelStatus {
                entry: e,
                downloaded,
                path: downloaded.then(|| path.to_string_lossy().to_string()),
            }
        })
        .collect();
    Ok(statuses)
}

/// Return only the IDs of models that are already downloaded.
#[tauri::command]
pub fn get_downloaded_models(app: AppHandle) -> Result<Vec<String>, MeetflowError> {
    let models_dir = storage::models_dir(&app)?;
    let ids = MODEL_CATALOG
        .iter()
        .filter(|e| models_dir.join(format!("ggml-{}.bin", e.id)).exists())
        .map(|e| e.id.to_string())
        .collect();
    Ok(ids)
}

/// Start downloading a Whisper model. Progress events are emitted to the frontend.
/// `model_id` must match a `ModelCatalogEntry.id`.
#[tauri::command]
pub async fn download_whisper_model(
    app: AppHandle,
    model_id: String,
) -> Result<String, MeetflowError> {
    let entry = MODEL_CATALOG
        .iter()
        .find(|e| e.id == model_id)
        .ok_or_else(|| MeetflowError::NotFound(format!("Model '{model_id}' not in catalog")))?;

    let client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(600)) // 10 min for large models
            .build()
            .map_err(|e| MeetflowError::Http(e.to_string()))?,
    );

    let path = download::download_model(app, entry, client).await?;
    Ok(path.to_string_lossy().to_string())
}

/// Cancel an in-progress download.
/// NOTE: Simple implementation — just deletes the partial file.
/// A proper cancellation token will be added in a future phase.
#[tauri::command]
pub async fn cancel_whisper_download(
    app: AppHandle,
    model_id: String,
) -> Result<(), MeetflowError> {
    let models_dir = storage::models_dir(&app)?;
    let partial = models_dir.join(format!("ggml-{model_id}.bin"));
    if partial.exists() {
        tokio::fs::remove_file(&partial).await?;
        tracing::info!(
            "Cancelled download, removed partial file: {}",
            partial.display()
        );
    }
    Ok(())
}

/// Transcribe a meeting's audio file using the first available Whisper model.
///
/// Called by the frontend when the meeting detail view opens with an empty
/// transcript. Runs whisper.cpp on a blocking thread so Tokio stays free.
/// Emits `transcript-ready` with the meeting ID when done.
#[tauri::command]
pub async fn transcribe_meeting(
    app: AppHandle,
    db: State<'_, DbPool>,
    meeting_id: String,
) -> Result<(), MeetflowError> {
    use rusqlite::OptionalExtension as _;

    // ── 1. Load meeting info from DB ─────────────────────────────────────────
    let (audio_path, already_transcribed) = {
        let conn = db
            .0
            .lock()
            .map_err(|_| MeetflowError::Db("Lock poisoned".into()))?;

        let audio_path: Option<String> = conn
            .query_row(
                "SELECT audio_path FROM meetings WHERE id = ?1",
                rusqlite::params![meeting_id],
                |row| row.get(0),
            )
            .optional()?
            .flatten();

        let content: Option<String> = conn
            .query_row(
                "SELECT content FROM transcripts WHERE meeting_id = ?1",
                rusqlite::params![meeting_id],
                |row| row.get(0),
            )
            .optional()?
            .flatten();

        let done = content.map(|c| !c.is_empty()).unwrap_or(false);
        (audio_path, done)
    };

    if already_transcribed {
        tracing::info!("Meeting {meeting_id} already has a transcript, skipping");
        return Ok(());
    }

    let audio_path = audio_path
        .ok_or_else(|| MeetflowError::NotFound("Meeting has no audio file".into()))?;

    // ── 2. Find first downloaded model ───────────────────────────────────────
    let models_dir = storage::models_dir(&app)?;
    let model_path = find_first_downloaded_model(&models_dir)?;

    // ── 3. Read WAV → Vec<f32> ───────────────────────────────────────────────
    let samples = read_wav_samples(&audio_path)?;

    // ── 4. Transcribe on blocking thread ────────────────────────────────────
    tracing::info!(
        "Starting transcription for meeting {meeting_id} ({} samples, model: {})",
        samples.len(),
        model_path.display()
    );

    let result = tokio::task::spawn_blocking(move || {
        WhisperEngine::transcribe_file(&model_path, &samples, None)
    })
    .await
    .map_err(|e| MeetflowError::Transcription(format!("Thread panicked: {e}")))?
    ?;

    // ── 5. Persist transcript ────────────────────────────────────────────────
    let segments_json = serde_json::to_string(&result.segments)
        .map_err(|e| MeetflowError::Db(format!("Segment serialization failed: {e}")))?;
    let word_count = result.text.split_whitespace().count() as i64;

    {
        let conn = db
            .0
            .lock()
            .map_err(|_| MeetflowError::Db("Lock poisoned".into()))?;

        conn.execute(
            "UPDATE transcripts SET content = ?1, segments = ?2, word_count = ?3
             WHERE meeting_id = ?4",
            rusqlite::params![result.text, segments_json, word_count, meeting_id],
        )?;

        conn.execute(
            "UPDATE meetings SET language = ?1 WHERE id = ?2",
            rusqlite::params![result.language, meeting_id],
        )?;
    }

    // ── 6. Notify frontend ───────────────────────────────────────────────────
    app.emit("transcript-ready", &meeting_id).ok();
    tracing::info!("Transcription complete for meeting {meeting_id}");
    Ok(())
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Return the path of the first downloaded model in catalog priority order.
fn find_first_downloaded_model(models_dir: &PathBuf) -> Result<PathBuf, MeetflowError> {
    for entry in MODEL_CATALOG {
        let path = models_dir.join(format!("ggml-{}.bin", entry.id));
        if path.exists() {
            tracing::info!("Using model: {}", path.display());
            return Ok(path);
        }
    }
    Err(MeetflowError::NotFound(
        "No Whisper model found. Download one in Onboarding → Model step.".into(),
    ))
}

/// Read a WAV file (any sample rate, mono or stereo, i16 or f32) and return
/// 16 kHz mono f32 samples ready for Whisper.
///
/// The recording pipeline already writes 16 kHz mono i16, so in practice no
/// resampling is needed. The function still handles other formats defensively.
fn read_wav_samples(path: &str) -> Result<Vec<f32>, MeetflowError> {
    let mut reader = hound::WavReader::open(path)
        .map_err(|e| MeetflowError::Storage(format!("Cannot open WAV '{path}': {e}")))?;

    let spec = reader.spec();

    let samples: Vec<f32> = match (spec.sample_format, spec.bits_per_sample) {
        (hound::SampleFormat::Int, 16) => reader
            .samples::<i16>()
            .map(|s| s.map(|x| x as f32 / 32_768.0))
            .collect::<Result<_, _>>()
            .map_err(|e| MeetflowError::Storage(e.to_string()))?,

        (hound::SampleFormat::Int, 32) => reader
            .samples::<i32>()
            .map(|s| s.map(|x| x as f32 / 2_147_483_648.0))
            .collect::<Result<_, _>>()
            .map_err(|e| MeetflowError::Storage(e.to_string()))?,

        (hound::SampleFormat::Float, 32) => reader
            .samples::<f32>()
            .collect::<Result<_, _>>()
            .map_err(|e| MeetflowError::Storage(e.to_string()))?,

        (fmt, bits) => {
            return Err(MeetflowError::Storage(format!(
                "Unsupported WAV format: {fmt:?} {bits}-bit"
            )));
        }
    };

    tracing::debug!(
        "WAV loaded: {} samples, {}Hz, {}ch",
        samples.len(),
        spec.sample_rate,
        spec.channels
    );

    Ok(samples)
}
