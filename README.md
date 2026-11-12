# GIWU AI Quantum Backend

A high-performance, modular Rust backend built with **Axum** and the **Rig** framework, designed to power the GIWU AI South African Authenticity & Cyber Fraud Inspection Engine.

---

## 🚀 Overview

The GIWU AI backend is a lightning-fast Rust microservice handling real-time text analysis, image-based scam/tender inspection, and catalog reference comparisons using specialized AI agents powered by Google Gemini.

---

## 📂 Project Structure

```text
giwu-ai-backend/
├── Cargo.toml          # Package metadata & crate dependencies
├── Cargo.lock          # Locked dependency versions
├── .env                # Environment configuration (Gemini API Key)
├── Dockerfile          # Multi-stage container build configuration
├── docker-compose.yml  # Docker Compose deployment setup
└── src/
    ├── main.rs         # Server entry point, router binding, and tracing
    ├── agents.rs       # Specialized Rig AI agents (Text, Job Scam, and Compare)
    ├── handlers.rs     # Axum multipart request handlers & routing logic
    └── models.rs       # Shared DTO structs (InspectionResult, RiskFlag, etc.)
