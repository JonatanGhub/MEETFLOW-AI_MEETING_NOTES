use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};

use crate::error::MeetflowError;

/// Information about an audio device exposed to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub kind: AudioDeviceKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AudioDeviceKind {
    Input,  // Microphone
    Output, // System audio (loopback)
}

/// Enumerate all available input (microphone) devices.
pub fn list_input_devices() -> Result<Vec<AudioDeviceInfo>, MeetflowError> {
    let host = cpal::default_host();
    let default_input = host.default_input_device();
    let default_name = default_input
        .as_ref()
        .and_then(|d| d.name().ok())
        .unwrap_or_default();

    let devices = host
        .input_devices()
        .map_err(|e| MeetflowError::Audio(e.to_string()))?
        .filter_map(|d| {
            let name = d.name().ok()?;
            let is_default = name == default_name;
            Some(AudioDeviceInfo {
                id: name.clone(),
                name: name.clone(),
                is_default,
                kind: AudioDeviceKind::Input,
            })
        })
        .collect();

    Ok(devices)
}

/// Enumerate system audio (loopback) devices.
/// On Windows, WASAPI exposes loopback variants of output devices.
pub fn list_loopback_devices() -> Result<Vec<AudioDeviceInfo>, MeetflowError> {
    let host = cpal::default_host();
    let default_output = host.default_output_device();
    let default_name = default_output
        .as_ref()
        .and_then(|d| d.name().ok())
        .unwrap_or_default();

    // Output devices can be used in loopback mode via WASAPI
    let devices = host
        .output_devices()
        .map_err(|e| MeetflowError::Audio(e.to_string()))?
        .filter_map(|d| {
            let name = d.name().ok()?;
            let is_default = name == default_name;
            Some(AudioDeviceInfo {
                id: format!("loopback:{name}"),
                name: format!("{name} (System audio)"),
                is_default,
                kind: AudioDeviceKind::Output,
            })
        })
        .collect();

    Ok(devices)
}
