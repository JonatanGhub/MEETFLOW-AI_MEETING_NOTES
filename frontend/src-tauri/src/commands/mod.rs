pub mod audio;
pub mod llm;
pub mod meetings;
pub mod settings;
pub mod whisper;

// Re-export all commands for the tauri::generate_handler! macro
pub use audio::{
    get_audio_devices,
    start_recording,
    stop_recording,
    pause_recording,
    resume_recording,
    get_recording_status,
};
pub use llm::{
    test_llm_connection,
    generate_meeting_summary,
    list_ollama_models,
};
pub use meetings::{
    list_meetings,
    get_meeting,
    update_meeting_title,
    delete_meeting,
    get_transcript,
    get_summary,
    get_note,
    save_note,
    export_meeting_markdown,
};
pub use settings::{
    get_setting,
    set_setting,
    get_app_data_dir,
    delete_all_data,
};
pub use whisper::{
    list_whisper_models,
    download_whisper_model,
    cancel_whisper_download,
    get_downloaded_models,
};
