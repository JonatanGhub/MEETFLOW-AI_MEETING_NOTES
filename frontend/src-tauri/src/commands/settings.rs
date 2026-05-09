use tauri::{AppHandle, State};

use crate::db::DbPool;
use crate::error::MeetflowError;
use crate::storage;

/// Get a settings value by key.
#[tauri::command]
pub fn get_setting(
    db: State<'_, DbPool>,
    key: String,
) -> Result<Option<String>, MeetflowError> {
    let conn = db.0.lock().map_err(|_| MeetflowError::Db("Lock poisoned".into()))?;
    let result = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        rusqlite::params![key],
        |row| row.get::<_, String>(0),
    );
    match result {
        Ok(v) => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Set (upsert) a settings value.
#[tauri::command]
pub fn set_setting(
    db: State<'_, DbPool>,
    key: String,
    value: String,
) -> Result<(), MeetflowError> {
    let conn = db.0.lock().map_err(|_| MeetflowError::Db("Lock poisoned".into()))?;
    let now = chrono::Utc::now().timestamp_millis();
    conn.execute(
        "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3",
        rusqlite::params![key, value, now],
    )?;
    Ok(())
}

/// Return the app data directory path (for display in Settings > Privacy).
#[tauri::command]
pub fn get_app_data_dir(app: AppHandle) -> Result<String, MeetflowError> {
    storage::app_data_dir(&app).map(|p| p.to_string_lossy().to_string())
}

/// Delete all user data: meetings, transcripts, notes, summaries, settings.
/// Audio files on disk are also removed.
#[tauri::command]
pub async fn delete_all_data(
    app: AppHandle,
    db: State<'_, DbPool>,
) -> Result<(), MeetflowError> {
    // Delete DB records
    {
        let conn = db.0.lock().map_err(|_| MeetflowError::Db("Lock poisoned".into()))?;
        conn.execute_batch(
            "DELETE FROM summaries;
             DELETE FROM transcripts;
             DELETE FROM notes;
             DELETE FROM meetings;
             DELETE FROM settings;",
        )?;
    }

    // Delete audio files
    let recordings_dir = storage::recordings_dir(&app)?;
    if let Ok(entries) = std::fs::read_dir(&recordings_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map(|e| e == "wav").unwrap_or(false) {
                tokio::fs::remove_file(entry.path()).await.ok();
            }
        }
    }

    tracing::info!("All user data deleted");
    Ok(())
}
