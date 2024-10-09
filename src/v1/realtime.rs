use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tungstenite::handshake::client::Request;
use url::Url;
use base64::encode;

#[derive(Serialize, Deserialize, Debug)]
struct RealtimeRequest {
    #[serde(rename = "type")]
    request_type: String,
    item: Option<Item>,
    response: Option<Response>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    modalities: Vec<String>,
    instructions: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    #[serde(rename = "type")]
    item_type: String,
    role: Option<String>,
    content: Vec<ItemContent>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
    audio: Option<String>,  // Base64 encoded PCM16 audio
}

#[derive(Serialize, Deserialize, Debug)]
struct RealtimeEvent {
    #[serde(rename = "type")]
    event_type: String,
    item_id: Option<String>,
    content_index: Option<usize>,
    delta: Option<String>,
    audio_delta: Option<String>,
    // Additional fields for different event types...
}

#[derive(Serialize, Deserialize, Debug)]
struct Session {
    id: String,
    object: String,
    model: String,
    voice: Option<String>,
}

pub async fn connect_realtime_api(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("wss://api.openai.com/v1/realtime?model=gpt-4o-realtime-preview-2024-10-01")?;
    let req = Request::builder()
        .uri(url.to_string())
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "realtime=v1")
        .body(())?;

    let (ws_stream, _) = connect_async(req).await.expect("Failed to connect");

    println!("Connected to the OpenAI Realtime API");

    let (mut write, mut read) = ws_stream.split();

    // Create a response request (can be text or audio response)
    let response_request = RealtimeRequest {
        request_type: "response.create".to_string(),
        response: Some(Response {
            modalities: vec!["text".to_string()],
            instructions: "Please assist the user.".to_string(),
        }),
        item: None,
    };

    // Send the response request to the API
    write.send(Message::Text(serde_json::to_string(&response_request)?)).await?;

    // Process incoming events from the WebSocket stream
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(msg)) => {
                let parsed: RealtimeEvent = serde_json::from_str(&msg)?;
                handle_realtime_event(parsed);
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed");
                break;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn handle_realtime_event(event: RealtimeEvent) {
    match event.event_type.as_str() {
        "conversation.item.created" => {
            // Handle item creation event
            println!("Item created: {:?}", event);
        }
        "response.audio.delta" => {
            // Handle receiving audio data from the server
            if let Some(audio_delta) = event.audio_delta {
                // Decode and process audio delta (base64 PCM16)
                let audio_data = decode_audio(&audio_delta);
                println!("Received audio: {:?}", audio_data);
            }
        }
        "response.text.delta" => {
            // Handle text responses from the server
            if let Some(delta) = event.delta {
                println!("Text delta: {}", delta);
            }
        }
        _ => {
            println!("Unknown event: {:?}", event);
        }
    }
}

// Utility function to convert base64 audio delta into PCM16
fn decode_audio(base64_audio: &str) -> Vec<i16> {
    let decoded_audio = base64::decode(base64_audio).expect("Failed to decode audio");
    decoded_audio
        .chunks(2)
        .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]))
        .collect()
}

// Utility function to encode raw PCM16 data into base64
fn encode_audio(audio_data: &[i16]) -> String {
    let pcm_bytes: Vec<u8> = audio_data.iter().flat_map(|&s| s.to_le_bytes()).collect();
    encode(pcm_bytes)
}

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    if let Err(e) = connect_realtime_api(&api_key).await {
        eprintln!("Error: {}", e);
    }
}
