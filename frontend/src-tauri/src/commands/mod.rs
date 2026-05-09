pub mod audio;
pub mod llm;
pub mod meetings;
pub mod settings;
pub mod whisper;

// Re-export all commands for the tauri::generate_handler! macro
pub use audio::{
    get_audio_devices, get_recording_status, pause_recording, resume_recording, start_recording,
    stop_recording,
};
pub use llm::{generate_meeting_summary, list_ollama_models, test_llm_connection};
pub use meetings::{
    delete_meeting, export_meeting_markdown, get_meeting, get_note, get_summary, get_transcript,
    list_meetings, save_note, update_meeting_title,
};
pub use settings::{delete_all_data, get_app_data_dir, get_setting, set_setting};
pub use whisper::{
    cancel_whisper_download, download_whisper_model, get_downloaded_models, list_whisper_models,
};
