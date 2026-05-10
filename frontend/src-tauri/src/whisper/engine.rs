use std::path::PathBuf;

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::db::models::TranscriptSegment;
use crate::error::MeetflowError;

pub struct WhisperEngine;

impl WhisperEngine {
    /// Transcribe 16 kHz mono f32 samples using the given ggml model file.
    ///
    /// This function is CPU-intensive and blocks the calling thread.
    /// Always call it via `tokio::task::spawn_blocking`.
    pub fn transcribe_file(
        model_path: &PathBuf,
        samples: &[f32],
        language: Option<&str>,
    ) -> Result<TranscribeResult, MeetflowError> {
        if !model_path.exists() {
            return Err(MeetflowError::Transcription(format!(
                "Model not found: {}",
                model_path.display()
            )));
        }

        let path_str = model_path.to_str().ok_or_else(|| {
            MeetflowError::Transcription("Model path contains non-UTF-8 characters".into())
        })?;

        tracing::info!(
            "Loading whisper model ({:.0} MB): {}",
            model_path.metadata().map(|m| m.len() / 1_048_576).unwrap_or(0),
            path_str
        );

        let ctx = WhisperContext::new_with_params(path_str, WhisperContextParameters::default())
            .map_err(|e| MeetflowError::Transcription(format!("Failed to load model: {e}")))?;

        let mut state = ctx
            .create_state()
            .map_err(|e| MeetflowError::Transcription(format!("Failed to create state: {e}")))?;

        let n_threads = std::thread::available_parallelism()
            .map(|n| n.get().min(8))
            .unwrap_or(4) as i32;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_n_threads(n_threads);
        params.set_language(language);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_special(false);
        params.set_suppress_blank(true);
        params.set_single_segment(false);

        tracing::info!(
            "Transcribing {:.1}s of audio with {} threads…",
            samples.len() as f32 / 16_000.0,
            n_threads
        );

        state
            .full(params, samples)
            .map_err(|e| MeetflowError::Transcription(format!("Transcription failed: {e}")))?;

        let n_segments = state.full_n_segments();

        let mut full_text = String::new();
        let mut segments: Vec<TranscriptSegment> = Vec::with_capacity(n_segments as usize);

        for i in 0..n_segments {
            let Some(seg) = state.get_segment(i) else {
                continue;
            };
            let seg_text = seg
                .to_str_lossy()
                .map_err(|e| MeetflowError::Transcription(e.to_string()))?;
            let seg_text = seg_text.trim().to_string();
            if seg_text.is_empty() {
                continue;
            }

            // Whisper timestamps are in centiseconds (1/100 s)
            let t0 = seg.start_timestamp() as f64 / 100.0;
            let t1 = seg.end_timestamp() as f64 / 100.0;

            if !full_text.is_empty() {
                full_text.push(' ');
            }
            full_text.push_str(&seg_text);

            segments.push(TranscriptSegment {
                start: t0,
                end: t1,
                text: seg_text,
                speaker: None,
            });
        }

        tracing::info!(
            "Transcription done: {} segments, {} chars",
            segments.len(),
            full_text.len()
        );

        Ok(TranscribeResult {
            text: full_text,
            segments,
            language: language.unwrap_or("auto").to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TranscribeResult {
    pub text: String,
    pub segments: Vec<TranscriptSegment>,
    pub language: String,
}
