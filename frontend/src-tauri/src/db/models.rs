use serde::{Deserialize, Serialize};

/// A recorded meeting session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub started_at: i64, // Unix ms
    pub ended_at: Option<i64>,
    pub duration_sec: Option<i64>,
    pub audio_path: Option<String>,
    pub language: Option<String>,
    pub created_at: i64,
}

/// Transcript associated with a meeting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transcript {
    pub id: String,
    pub meeting_id: String,
    pub content: String,
    pub segments: Vec<TranscriptSegment>,
    pub word_count: i64,
    pub created_at: i64,
}

/// One timed segment of a transcript.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSegment {
    pub start: f64, // seconds from recording start
    pub end: f64,
    pub text: String,
    pub speaker: Option<String>, // v0.2+ with sherpa-onnx
}

/// AI-generated summary for a meeting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub id: String,
    pub meeting_id: String,
    pub executive_summary: Option<String>,
    pub action_items: Vec<ActionItem>,
    pub topics: Vec<String>,
    pub sentiment: Option<String>,
    pub score: Option<i64>,
    pub provider: String,
    pub model: String,
    pub created_at: i64,
}

/// A single action item extracted from the meeting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionItem {
    pub text: String,
    pub assignee: Option<String>,
    pub due: Option<String>, // ISO 8601 date string
    pub done: bool,
}

/// Block-based notes for a meeting (BlockNote JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub meeting_id: String,
    pub content: String, // BlockNote JSON array serialized to string
    pub updated_at: i64,
}

/// Abbreviated meeting card shown in the sidebar/list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingCard {
    pub id: String,
    pub title: String,
    pub started_at: i64,
    pub duration_sec: Option<i64>,
    pub score: Option<i64>,
    pub action_item_count: usize,
    pub summary_snippet: Option<String>, // first 120 chars of executive_summary
}
