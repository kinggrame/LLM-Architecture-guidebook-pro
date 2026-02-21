pub mod scene;

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

use scene::{Scene, VisualNovelScene};

/// Scene Manager handles scene transitions and lifecycle
pub struct SceneManager {
    scenes: HashMap<String, Arc<dyn Scene>>,
    scene_stack: Vec<Arc<dyn Scene>>,
}

impl SceneManager {
    /// Create a new scene manager
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            scene_stack: Vec::new(),
        }
    }

    /// Initialize the scene manager
    pub fn initialize(&mut self) -> Result<()> {
        log::info!("SceneManager initialized");
        Ok(())
    }

    /// Shutdown the scene manager
    pub fn shutdown(&mut self) {
        while let Some(scene) = self.scene_stack.pop() {
            scene.on_exit();
        }
        self.scenes.clear();
        log::info!("SceneManager shutdown");
    }

    /// Register a scene
    pub fn register_scene<S: Scene + 'static>(&mut self, name: &str, scene: S) {
        self.scenes.insert(name.to_string(), Arc::new(scene));
    }

    /// Change to a different scene (clears the stack)
    pub fn change_scene(&mut self, name: &str) {
        if let Some(scene) = self.scenes.get(name).cloned() {
            // Exit current scene
            if let Some(current) = self.scene_stack.pop() {
                current.on_exit();
            }
            
            // Clear stack and push new scene
            self.scene_stack.clear();
            scene.on_enter();
            self.scene_stack.push(scene);
            
            log::debug!("Changed to scene: {}", name);
        } else {
            log::warn!("Scene not found: {}", name);
        }
    }

    /// Push a scene onto the stack
    pub fn push_scene(&mut self, name: &str) {
        if let Some(scene) = self.scenes.get(name).cloned() {
            // Exit current scene
            if let Some(current) = self.scene_stack.last() {
                current.on_exit();
            }
            
            // Push new scene
            scene.on_enter();
            self.scene_stack.push(scene);
            
            log::debug!("Pushed scene: {}", name);
        } else {
            log::warn!("Scene not found: {}", name);
        }
    }

    /// Pop the current scene
    pub fn pop_scene(&mut self) {
        if let Some(scene) = self.scene_stack.pop() {
            scene.on_exit();
            
            // Enter previous scene
            if let Some(prev) = self.scene_stack.last() {
                prev.on_enter();
            }
        }
    }

    /// Update the current scene
    pub fn update(&self, delta_time: f32) {
        if let Some(scene) = self.scene_stack.last() {
            scene.on_update(delta_time);
        }
    }

    /// Render the current scene
    pub fn render(&self) {
        if let Some(scene) = self.scene_stack.last() {
            scene.on_render();
        }
    }

    /// Get the current scene
    pub fn current_scene(&self) -> Option<Arc<dyn Scene>> {
        self.scene_stack.last().cloned()
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
