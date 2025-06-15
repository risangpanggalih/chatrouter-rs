use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>, // Optional, as it's specified in the URL
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>, // Optional for Gemini
}

#[derive(Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

// Legacy structure for backward compatibility with OpenRouter format
#[derive(Serialize, Deserialize)]
pub struct LegacyBody {
    pub messages: Vec<LegacyMessage>,
    pub max_tokens: Option<i32>,
    pub model: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LegacyMessage {
    pub role: String,
    pub content: String,
}

// Convert legacy format to Gemini format
pub fn convert_legacy_to_gemini(legacy: &LegacyBody) -> Body {
    let contents = legacy.messages.iter().map(|msg| {
        Content {
            parts: vec![Part { text: msg.content.clone() }],
            role: Some(msg.role.clone()),
        }
    }).collect();

    Body {
        contents,
        model: legacy.model.clone(),
    }
}

pub async fn send(body: &Body) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    // Ensure .env is loaded (in case this function is called independently)
    dotenv::dotenv().ok();

    let api_key =
        env::var("GEMINI_API_KEY").map_err(|e| format!("GEMINI_API_KEY not set: {}", e))?;

    // Default model if not specified
    let model = body.model.as_deref().unwrap_or("gemini-2.0-flash");

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(body)
        .send()
        .await?;

    println!("Gemini Response Status: {}", response.status());

    if response.status().is_success() {
        let resp_text = response.text().await?;
        println!("Gemini Response Body: {}", resp_text);
        Ok(resp_text) // Return the response body string
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Could not read error body".to_string());
        println!("Gemini Error Response Body: {}", error_text);
        Err(format!(
            "Gemini request failed with status {}: {}",
            status, error_text
        )
        .into())
    }
}

// Legacy send function for backward compatibility
pub async fn send_legacy(legacy_body: &LegacyBody) -> Result<String, Box<dyn std::error::Error>> {
    let gemini_body = convert_legacy_to_gemini(legacy_body);
    send(&gemini_body).await
}
