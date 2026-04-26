# Voice Pipeline

The core interaction loop: user speaks, Rune listens, sends to AI, plays the response.

## User Flow

```
User presses button
       │
       ▼
Display: "Listening..."
LED on (if present)
       │
       ▼
Record 16kHz mono PCM from I2S mic
       │
       ▼
Open WebSocket to AI provider
Stream audio chunks
       │
       ▼
Display: "Thinking..."
       │
       ▼
Receive text + audio response
       │
       ▼
Display: render text to e-ink
Display: "Speaking..."
Play audio through I2S amp
       │
       ▼
Display: show response text (idle state)
Wait for next button press
```

## Data Flow Diagram

```
┌─────────┐     PCM      ┌──────────────┐    WebSocket     ┌──────────────┐
│ SPH0645 │──────────────▶│              │─────────────────▶│              │
│  (mic)  │   I2S 16kHz   │  Rune Main   │  audio chunks    │  AI Provider │
└─────────┘    mono S32    │  Process     │                  │  (OpenAI RT  │
                           │              │◀─────────────────│   or other)  │
┌─────────┐     PCM        │              │  text + audio    │              │
│MAX98357A│◀──────────────│              │  events           │              │
│  (amp)  │   I2S playback │              │                  └──────────────┘
└─────────┘                │              │
                           │              │     SPI
┌─────────┐                │              │──────────────────▶┌─────────────┐
│ Button  │───────────────▶│              │  framebuffer      │  E-Ink      │
│  GPIO   │  trigger        └──────────────┘  updates          │  Display    │
└─────────┘                                                    └─────────────┘
```

## AI Provider Configuration

### OpenAI Realtime API (Default)

The OpenAI Realtime API is the default backend. It accepts streaming audio and returns both text and audio responses over a single WebSocket connection.

Endpoint: `wss://api.openai.com/v1/realtime`

### Configurable Backend

Rune does not hard-code OpenAI. The provider is configured via environment variables or a config file:

```toml
# /etc/rune/config.toml

[voice]
provider_url = "wss://api.openai.com/v1/realtime"
api_key_env = "OPENAI_API_KEY"   # name of env var holding the key
model = "gpt-4o-realtime-preview"

# To use a different provider, change the URL and auth:
# provider_url = "wss://your-provider.com/v1/audio"
# api_key_env = "CUSTOM_API_KEY"
```

The API key is stored in an environment variable, not in the config file. Set it on the device:

```bash
echo 'export OPENAI_API_KEY="sk-..."' >> /etc/profile
```

Any WebSocket API that accepts PCM audio and returns text/audio events can be used. The protocol adapter layer in Rune's code handles the translation.

## Audio Capture

### Recording from the Mic

```rust
// Pseudocode — actual implementation uses ALSA bindings
fn start_recording(tx: Sender<AudioChunk>) {
    let device = AlsaDevice::open("hw:0,0", Direction::Capture);
    device.set_format(Format::S32LE);  // SPH0645 outputs 24-bit in 32-bit frame
    device.set_rate(16000);
    device.set_channels(1);

    loop {
        let pcm_data = device.read_frames(1600);  // 100ms at 16kHz
        // Convert S32 to S16 (drop lower bits, shift right 16)
        let s16_data = convert_s32_to_s16(pcm_data);
        tx.send(AudioChunk { data: s16_data, timestamp: now() });
    }
}
```

### Audio Chunking

Audio is captured continuously while the user speaks and sent in chunks:

- Chunk size: 100ms (1600 samples at 16kHz)
- Format: 16-bit signed PCM, little-endian, mono
- The SPH0645 outputs 32-bit frames with 24-bit data. Convert to 16-bit before sending.

### Voice Activity Detection (Optional)

For a better UX, implement simple voice activity detection (VAD):

1. Compute RMS energy of each chunk.
2. If energy exceeds threshold, user is speaking. Keep recording.
3. If energy drops below threshold for N chunks (e.g., 1 second of silence), stop recording.

This avoids requiring the user to hold a button for the entire duration. Press to start, release when done (or auto-detect silence).

## WebSocket Connection

### Connecting to OpenAI Realtime API

```rust
// Pseudocode
fn connect_to_provider(config: &Config) -> WebSocket {
    let url = &config.voice.provider_url;
    let api_key = env::var(&config.voice.api_key_env).unwrap();

    let request = Request::builder()
        .uri(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "realtime=v1")
        .body(())
        .unwrap();

    let (ws, _response) = connect(request).unwrap();
    ws
}
```

### Sending Audio

```rust
// Send session config first
ws.send(json!({
    "type": "session.update",
    "session": {
        "modalities": ["text", "audio"],
        "instructions": "You are Rune, a pocket AI companion. Be concise.",
        "input_audio_format": "pcm16",
        "output_audio_format": "pcm16",
        "input_audio_transcription": { "model": "whisper-1" },
        "turn_detection": { "type": "server_vad" }
    }
}));

// Stream audio chunks as they're captured
for chunk in audio_rx {
    let b64_audio = base64::encode(&chunk.data);
    ws.send(json!({
        "type": "input_audio_buffer.append",
        "audio": b64_audio
    }));
}

// Signal end of input
ws.send(json!({
    "type": "input_audio_buffer.commit"
}));
ws.send(json!({
    "type": "response.create"
}));
```

### Receiving Response

```rust
loop {
    let msg = ws.read().unwrap();
    let event: Value = serde_json::from_str(&msg).unwrap();

    match event["type"].as_str() {
        Some("response.audio_transcript.delta") => {
            // Partial text — update display
            let text = event["delta"].as_str().unwrap();
            display.append_text(text);
        }
        Some("response.audio.delta") => {
            // Audio chunk — queue for playback
            let audio_b64 = event["delta"].as_str().unwrap();
            let audio_pcm = base64::decode(audio_b64).unwrap();
            speaker.queue_audio(audio_pcm);
        }
        Some("response.audio.done") => {
            // Audio stream complete
            speaker.flush();
        }
        Some("response.done") => {
            // Full response complete
            display.set_status("idle");
            break;
        }
        Some("error") => {
            let msg = event["error"]["message"].as_str().unwrap_or("unknown error");
            display.show_error(msg);
            break;
        }
        _ => {}
    }
}
```

## Display Status Indicators

The e-ink display shows the current state:

| State | Display |
|-------|---------|
| Idle | Last response text, or default screen |
| Listening | "Listening..." with a visual indicator |
| Thinking | "Thinking..." (waiting for AI response) |
| Speaking | Response text + "Speaking..." indicator |
| Error | Error message (network timeout, API error, etc.) |

Use partial refresh for status transitions — they're fast enough (200-300ms) to feel responsive.

## Audio Playback

### Playing the Response

```rust
fn start_playback(rx: Receiver<AudioChunk>) {
    let device = AlsaDevice::open("hw:0,0", Direction::Playback);
    device.set_format(Format::S16LE);
    device.set_rate(24000);   // OpenAI outputs 24kHz
    device.set_channels(1);

    for chunk in rx {
        device.write_frames(&chunk.data);
    }
    device.drain();  // Wait for all audio to play
}
```

Note: OpenAI Realtime API outputs audio at 24kHz. The T113 I2S must be configured to support this sample rate for playback, or you need to resample.

### Streaming Playback

Don't wait for the full response before playing. Start playback as soon as the first audio chunk arrives. This reduces perceived latency significantly.

```
Timeline:
  [capture]──▶[send]──▶[wait]──▶[receive chunk 1]──▶[play chunk 1]
                                 [receive chunk 2]──▶[play chunk 2]
                                 [receive chunk 3]──▶[play chunk 3]
                                        ...
```

## Privacy

Rune has no cloud backend of its own.

- Audio goes directly from the device to the AI provider the user configured.
- No Rune servers sit in the middle.
- No telemetry, analytics, or usage tracking.
- The API key belongs to the user, on the user's account.
- Audio is not stored by Rune after the session ends. Whether the AI provider stores it depends on their policies.

The user is in full control of where their data goes. Rune is the pipe, not the destination.

## Error Handling

### Network Timeout

```
If WebSocket connection fails or times out:
  → Display: "No connection. Check WiFi."
  → Retry with exponential backoff (1s, 2s, 4s, max 30s)
```

### API Error

```
If AI provider returns an error:
  → Display the error message on e-ink
  → Common: rate limit (429), auth failure (401), server error (500)
```

### Mic Failure

```
If ALSA device open fails:
  → Display: "Mic error. Check audio hardware."
  → Log details to /var/log/rune/
```

### Speaker Failure

```
If playback device fails:
  → Still display the text response on e-ink
  → Log the audio error
```

The text response is always useful even if audio fails. Display it regardless.

## Implementation Order

1. **Basic capture**: record audio to a file, verify it sounds right.
2. **Basic playback**: play a pre-recorded file through the speaker.
3. **WebSocket connection**: connect to the API, send a text prompt, receive a text response. Display on e-ink.
4. **Streaming audio**: send captured audio over WebSocket, play response audio.
5. **Full loop**: button press → record → send → display + play → idle.
6. **Polish**: VAD, streaming playback, error handling, status indicators.
