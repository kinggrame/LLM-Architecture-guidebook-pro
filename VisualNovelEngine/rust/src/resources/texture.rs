use anyhow::{Context, Result};
use image::DynamicImage;
use std::path::Path;

/// Texture wrapper for images
pub struct Texture {
    path: String,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Texture {
    /// Load a texture from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let img = image::open(path_ref)
            .with_context(|| format!("Failed to load texture: {:?}", path_ref))?;
        
        let (width, height) = (img.width(), img.height());
        let data = img.to_rgba8().into_raw();
        
        log::debug!("Loaded texture: {:?} ({}x{})", path_ref, width, height);
        
        Ok(Self {
            path: path_ref.to_string_lossy().to_string(),
            width,
            height,
            data,
        })
    }

    /// Get texture width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get texture height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get raw texture data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get texture path
    pub fn path(&self) -> &str {
        &self.path
    }
}
