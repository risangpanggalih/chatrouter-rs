# ChatRouter RS (Gemini API)

A basic chat router built in Rust that forwards chat requests to Google's Gemini AI API.

## Setup

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Configure Environment Variables**:
   - Copy `.env.example` to `.env`:
     ```bash
     cp .env.example .env
     ```
   - Edit `.env` and add your Gemini API key:
     ```
     GEMINI_API_KEY=your_actual_api_key_here
     ```
   - Get your API key from: https://aistudio.google.com/app/apikey

3. **Run the server**:
   ```bash
   cargo run
   ```

## Usage

The server runs on `localhost:8080` and accepts POST requests with JSON payloads in two formats:

### Gemini Format (Native):
```json
{
  "contents": [
    {
      "parts": [
        {
          "text": "Explain how AI works in a few words"
        }
      ]
    }
  ]
}
```

### Legacy Format (OpenRouter/ChatGPT compatible):
```json
{
  "messages": [
    {
      "role": "user",
      "content": "Hello, how are you?"
    }
  ],
  "max_tokens": 100,
  "model": "gemini-2.0-flash"
}
```

### Example with curl (Gemini format):
```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [
      {
        "parts": [
          {
            "text": "Explain how AI works in a few words"
          }
        ]
      }
    ]
  }'
```

### Example with curl (Legacy format):
```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "messages": [
      {
        "role": "user",
        "content": "Hello, how are you?"
      }
    ],
    "model": "gemini-2.0-flash"
  }'
```

## Features

- **Dual Format Support**: Accepts both Gemini native format and legacy OpenRouter/ChatGPT format
- **Automatic Detection**: Automatically detects and converts between formats
- **Environment Configuration**: Uses `.env` files for secure API key management
- **Error Handling**: Comprehensive error handling and logging

## To do:
- Add postgres + pgvector interaction
- Add RAG logic
- Set dockerfile
