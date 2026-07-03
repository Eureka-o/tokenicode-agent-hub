// Enhanced usage record with runtime/provider dimensions
//
// Phase 1.7 will add logic to populate and persist these records.
// For now this is a pure data type.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageRecord {
    pub session_id: String,
    pub runtime_id: String,      // "claude" | "codex" | etc
    pub runtime_name: String,
    pub provider_id: String,
    pub provider_name: String,
    pub model: String,
    pub project_path: String,
    pub timestamp: i64,
    pub hour: u8,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_tokens: u64,
    pub total_tokens: u64,
    pub message_count: u32,
    pub duration_ms: u64,
    pub estimated_cost: Option<f64>,
    pub is_estimated: bool,
}
