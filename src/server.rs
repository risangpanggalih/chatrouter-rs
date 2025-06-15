use crate::utils::{Body, LegacyBody, send, send_legacy};
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde_json::Value;
use std::env;

// Handle requests with automatic format detection
async fn handle_request(body: web::Json<Value>) -> impl Responder {
    let body_value = body.into_inner();

    // Try to detect the format and handle accordingly
    let result = if body_value.get("contents").is_some() {
        // Gemini format
        match serde_json::from_value::<Body>(body_value) {
            Ok(gemini_body) => send(&gemini_body).await,
            Err(e) => {
                eprintln!("Error parsing Gemini format: {}", e);
                return HttpResponse::BadRequest()
                    .body("Invalid Gemini format");
            }
        }
    } else if body_value.get("messages").is_some() {
        // Legacy OpenRouter/ChatGPT format
        match serde_json::from_value::<LegacyBody>(body_value) {
            Ok(legacy_body) => send_legacy(&legacy_body).await,
            Err(e) => {
                eprintln!("Error parsing legacy format: {}", e);
                return HttpResponse::BadRequest()
                    .body("Invalid legacy format");
            }
        }
    } else {
        eprintln!("Unknown request format");
        return HttpResponse::BadRequest()
            .body("Request must contain either 'contents' (Gemini format) or 'messages' (legacy format)");
    };

    match result {
        Ok(response) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(response) // Send the raw JSON string back
        }
        Err(e) => {
            eprintln!("Error forwarding request to Gemini: {}", e);
            HttpResponse::InternalServerError()
                .body("Failed to communicate with Gemini API.")
        }
    }
}

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    env::var("GEMINI_API_KEY")
        .expect("Please set the GEMINI_API_KEY environment variable in .env file or as an environment variable.");

    println!("Starting server on localhost:8080...");
    HttpServer::new(|| {
        App::new()
            // Ensure the route expects JSON
            .route("/", web::post().to(handle_request))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
