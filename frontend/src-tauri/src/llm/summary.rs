use serde::{Deserialize, Serialize};

use crate::db::models::{ActionItem, Summary};
use crate::error::MeetflowError;

use super::client::LlmClient;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSummaryRequest {
    pub meeting_id: String,
    pub transcript: String,
    pub meeting_title: String,
    pub duration_sec: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSummaryResponse {
    pub executive_summary: String,
    pub action_items: Vec<ActionItem>,
    pub topics: Vec<String>,
    pub sentiment: String,
    pub score: i64,
}

// ─── Summary generation ───────────────────────────────────────────────────────

pub async fn generate_summary(
    client: &LlmClient,
    req: &GenerateSummaryRequest,
) -> Result<GenerateSummaryResponse, MeetflowError> {
    let system = SUMMARY_SYSTEM_PROMPT;

    let user = format!(
        "Meeting: {title}\nDuration: {duration}\n\nTranscript:\n{transcript}",
        title = req.meeting_title,
        duration = req
            .duration_sec
            .map(|s| format!("{} min", s / 60))
            .unwrap_or_else(|| "unknown".to_string()),
        transcript = truncate_transcript(&req.transcript, 12_000),
    );

    let raw = client.complete(system, &user).await?;

    parse_summary_response(&raw)
        .map_err(|e| MeetflowError::Llm(format!("Failed to parse summary JSON: {e}")))
}

fn truncate_transcript(text: &str, max_chars: usize) -> &str {
    if text.len() <= max_chars {
        text
    } else {
        // Cut at last word boundary
        let cut = &text[..max_chars];
        cut.rfind(char::is_whitespace).map(|i| &text[..i]).unwrap_or(cut)
    }
}

fn parse_summary_response(raw: &str) -> Result<GenerateSummaryResponse, serde_json::Error> {
    // Extract JSON block if wrapped in markdown code fences
    let json_str = if let (Some(start), Some(end)) = (raw.find("```json"), raw.rfind("```")) {
        raw[start + 7..end].trim()
    } else if let (Some(start), Some(end)) = (raw.find('{'), raw.rfind('}')) {
        &raw[start..=end]
    } else {
        raw.trim()
    };

    serde_json::from_str(json_str)
}

const SUMMARY_SYSTEM_PROMPT: &str = r#"You are a meeting intelligence assistant. Analyze meeting transcripts and return a structured JSON summary.

Return ONLY valid JSON with this exact structure:
{
  "executiveSummary": "2-4 sentence summary of what was discussed and decided",
  "actionItems": [
    {"text": "action description", "assignee": "person name or null", "due": "YYYY-MM-DD or null", "done": false}
  ],
  "topics": ["topic 1", "topic 2"],
  "sentiment": "positive" | "neutral" | "negative",
  "score": 0-100
}

Score rubric: 100 = short, focused, clear decisions + action items. Deduct for: no action items (-20), vague outcomes (-15), very long without resolution (-10).
Extract action items only when explicitly agreed in the meeting.
Keep executiveSummary concise and factual."#;

/// Convert a `GenerateSummaryResponse` to the DB `Summary` model.
pub fn to_db_summary(
    response: GenerateSummaryResponse,
    meeting_id: String,
    provider: String,
    model: String,
) -> Summary {
    Summary {
        id: uuid::Uuid::new_v4().to_string(),
        meeting_id,
        executive_summary: Some(response.executive_summary),
        action_items: response.action_items,
        topics: response.topics,
        sentiment: Some(response.sentiment),
        score: Some(response.score),
        provider,
        model,
        created_at: chrono::Utc::now().timestamp_millis(),
    }
}
