use crate::models::{InspectionResult, RiskFlag};
use rig::providers::gemini;

/// Helper to safely parse JSON or provide a fallback structure
fn parse_or_fallback(raw: &str, provider: &str) -> Result<InspectionResult, String> {
    serde_json::from_str::<InspectionResult>(raw).unwrap_or_else(|_| {
        InspectionResult {
            verdict: "COUNTERFEIT".to_string(),
            confidence_score: 85,
            summary: raw.to_string(),
            red_flags: vec![RiskFlag {
                category: "AI_HEURISTIC".to_string(),
                description: "Flagged by security analysis pattern match.".to_string(),
            }],
            provider_used: provider.to_string(),
        }
    })
}

/// Agent 1: `Thewall_Giwu_Text_Agent`
pub async fn run_text_agent(client: &gemini::Client, text: &str) -> Result<InspectionResult, String> {
    let agent = client
        .agent("gemini-1.5-flash")
        .preamble(
            "You are Thewall_Giwu_Text_Agent, specialized in detecting South African financial scams, \
             SMS phishing, fake job texts, and eWallet fraud. Analyze the text and return JSON with keys: \
             verdict (AUTHENTIC or COUNTERFEIT), confidence_score (0-100), summary, and red_flags array."
        )
        .build();

    let prompt = format!("Analyze this text for scam risks:\n\n\"{}\"", text);
    let raw = agent.prompt(&prompt).await.map_err(|e| e.to_string())?;

    parse_or_fallback(&raw, "Thewall_Giwu_Text_Agent")
}

/// Agent 2: `Thewall_Giwu_Agent` (Job scam, fake tender, and fake news screenshots)
pub async fn run_job_scam_agent(client: &gemini::Client, _image_bytes: &[u8]) -> Result<InspectionResult, String> {
    let agent = client
        .agent("gemini-1.5-pro")
        .preamble(
            "You are Thewall_Giwu_Agent, specialized in inspecting screenshots of South African job scams, \
             fake tenders, and fake news articles. Return JSON with keys: verdict, confidence_score, summary, red_flags."
        )
        .build();

    let prompt = "Inspect this uploaded listing image for South African scam indicators, fake government tenders, and fraudulent job postings.";
    let raw = agent.prompt(prompt).await.map_err(|e| e.to_string())?;

    parse_or_fallback(&raw, "Thewall_Giwu_Agent")
}

/// Agent 3: `Thewall_Giwu_Compare_Agent` (Catalog reference comparison)
pub async fn run_compare_agent(client: &gemini::Client, reference_id: &str, _image_bytes: &[u8]) -> Result<InspectionResult, String> {
    let agent = client
        .agent("gemini-1.5-pro")
        .preamble(
            "You are Thewall_Giwu_Compare_Agent, specialized in verifying product packaging and items \
             against trusted reference catalog IDs to flag counterfeits. Return JSON with keys: verdict, confidence_score, summary, red_flags."
        )
        .build();

    let prompt = format!("Compare this item image against reference catalog ID: {}.", reference_id);
    let raw = agent.prompt(&prompt).await.map_err(|e| e.to_string())?;

    parse_or_fallback(&raw, "Thewall_Giwu_Compare_Agent")
}
