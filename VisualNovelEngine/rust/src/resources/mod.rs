pub mod texture;
pub mod audio;

use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Weak};

use texture::Texture;
use audio::AudioClip;

/// Resource Manager handles loading and caching of game assets
pub struct ResourceManager {
    base_path: String,
    textures: HashMap<String, Weak<Texture>>,
    audio: HashMap<String, Weak<AudioClip>>,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            base_path: "assets/".to_string(),
            textures: HashMap::new(),
            audio: HashMap::new(),
        }
    }

    /// Initialize the resource manager
    pub fn initialize(&mut self) -> Result<()> {
        log::info!("ResourceManager initialized");
        Ok(())
    }

    /// Shutdown the resource manager
    pub fn shutdown(&mut self) {
        self.clear();
        log::info!("ResourceManager shutdown");
    }

    /// Load a texture from file
    pub fn load_texture(&mut self, path: &str) -> Result<Arc<Texture>> {
        let full_path = format!("{}{}", self.base_path, path);
        
        // Check if already loaded
        if let Some(weak) = self.textures.get(path) {
            if let Some(texture) = weak.upgrade() {
                return Ok(texture);
            }
        }
        
        // Load new texture
        let texture = Arc::new(Texture::load(&full_path)?);
        self.textures.insert(path.to_string(), Arc::downgrade(&texture));
        Ok(texture)
    }

    /// Get an existing texture
    pub fn get_texture(&self, path: &str) -> Option<Arc<Texture>> {
        self.textures.get(path).and_then(|w| w.upgrade())
    }

    /// Load an audio clip from file
    pub fn load_audio(&mut self, path: &str) -> Result<Arc<AudioClip>> {
        let full_path = format!("{}{}", self.base_path, path);
        
        // Check if already loaded
        if let Some(weak) = self.audio.get(path) {
            if let Some(audio) = weak.upgrade() {
                return Ok(audio);
            }
        }
        
        // Load new audio
        let audio = Arc::new(AudioClip::load(&full_path)?);
        self.audio.insert(path.to_string(), Arc::downgrade(&audio));
        Ok(audio)
    }

    /// Get an existing audio clip
    pub fn get_audio(&self, path: &str) -> Option<Arc<AudioClip>> {
        self.audio.get(path).and_then(|w| w.upgrade())
    }

    /// Clear all cached resources
    pub fn clear(&mut self) {
        self.textures.clear();
        self.audio.clear();
    }

    /// Set the base path for assets
    pub fn set_base_path(&mut self, path: &str) {
        self.base_path = path.to_string();
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}
