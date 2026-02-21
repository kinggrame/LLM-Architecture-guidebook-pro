use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

/// Audio clip that can be played
pub struct AudioClip {
    path: String,
    sink: Option<Sink>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
}

impl AudioClip {
    /// Load an audio clip from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let (stream, stream_handle) = OutputStream::try_default()
            .context("Failed to create audio output stream")?;
        
        log::debug!("Loaded audio: {:?}", path_ref);
        
        Ok(Self {
            path: path_ref.to_string_lossy().to_string(),
            sink: None,
            _stream: stream,
            _stream_handle: stream_handle,
        })
    }

    /// Play the audio clip
    pub fn play(&mut self) -> Result<()> {
        let file = File::open(&self.path)
            .with_context(|| format!("Failed to open audio file: {}", self.path))?;
        
        let decoder = Decoder::new(BufReader::new(file))
            .context("Failed to decode audio")?;
        
        self.sink = Some(Sink::try_new(&self._stream_handle)?);
        if let Some(sink) = &self.sink {
            sink.append(decoder);
        }
        
        Ok(())
    }

    /// Stop playback
    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }

    /// Pause playback
    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    /// Resume playback
    pub fn resume(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    /// Set volume (0.0 - 1.0)
    pub fn set_volume(&mut self, volume: f32) {
        if let Some(sink) = &self.sink {
            sink.set_volume(volume);
        }
    }

    /// Set looping
    pub fn set_loop(&mut self, _looping: bool) {
        // Note: rodio doesn't have direct looping support for Sink
        // Would need to implement manually for full support
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map_or(false, |s| !s.is_paused() && !s.empty())
    }

    /// Get audio path
    pub fn path(&self) -> &str {
        &self.path
    }
}
