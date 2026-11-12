use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFlag {
    pub category: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionResult {
    pub verdict: String, // "AUTHENTIC" or "COUNTERFEIT"
    pub confidence_score: u8,
    pub summary: String,
    pub red_flags: Vec<RiskFlag>,
    pub provider_used: String,
}

#[derive(Clone)]
pub struct AppState {
    pub gemini_client: rig::providers::gemini::Client,
}
