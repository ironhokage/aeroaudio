# AeroAudio

**AeroAudio** – A retro-futuristic terminal-based music hub for Windows (and potentially Android), designed to **play, mix, and manipulate music** in real-time with stunning ASCII/graphical visualizations, charts, and interactive controls.

AeroAudio is not just a music player – it’s a **creative playground** for musicians, coders, and audio enthusiasts who love seeing music data come alive in a terminal environment.

---

## Features

### Core Features
- Terminal-based, retro-futuristic UI with customizable themes and color schemes
- Multi-format music playback (MP3, FLAC, WAV, OGG, etc.) using **Rodio**
- Real-time waveform, spectrogram, and frequency graphs with ASCII/ANSI graphics
- Playlist management and track queuing
- Crossfade, loop, and shuffle playback modes
- Volume, pan, and speed control

### Advanced Tools
- Audio mixer: combine multiple tracks simultaneously with individual volume and effects
- Track manipulation: pitch shift, tempo adjust, reverse, and filters
- Real-time visualizations: spectrogram, bar graph, waveform, and VU meters
- Recording and exporting sessions
- Integration with external MIDI or OSC controllers (future expansion)

### Analytics & Visualization
- Detailed track metadata display: BPM, key, duration, codec info
- Dynamic charts for frequency analysis, beats, and amplitude trends
- Interactive terminal charts for live audio feedback

---

## Tech Stack

| Component             | Purpose                                         |
|-----------------------|-------------------------------------------------|
| **Rust**              | High-performance backend for audio processing and low-level control |
| **Rodio**             | Audio playback library for Rust                |
| **Symphonia**         | Audio decoding and sample access               |
| **Egui / Iced / Druid** | Optional GUI/terminal hybrid for charts and controls |
| **ASCII / ANSI graphics** | Terminal visualization of audio data        |
| **Python (optional)** | Rapid prototyping for advanced graphing/visualizations |

**Design Philosophy:**  
- High performance and low-latency playback  
- Modularity: backend separated from UI and visualizations  
- Extensible: easy to add new audio effects or visualization types  

---

## UX & Interface

### Terminal Experience
- Retro-futuristic aesthetic inspired by cyberpunk and old-school terminal apps
- Interactive keyboard controls for navigation and playback
- Multi-pane terminal layout: playlist, track info, waveform, spectrogram
- Real-time visual feedback with ASCII charts and colors

### Control Mapping
- Arrow keys / WASD → Navigate playlists
- Space → Play / Pause
- `+` / `-` → Volume control
- Numbers → Switch tracks
- Custom key-bindings for advanced features (mixing, effects)

