use crate::core::config::GameConfig;
use crate::rendering::RenderEngine;
use crate::resources::ResourceManager;
use crate::scene::SceneManager;
use crate::script::ScriptEngine;
use anyhow::Result;
use std::sync::Arc;
use parking_lot::Mutex;

/// Main engine struct that coordinates all subsystems
pub struct Engine {
    config: GameConfig,
    resource_manager: Arc<Mutex<ResourceManager>>,
    render_engine: Arc<Mutex<RenderEngine>>,
    scene_manager: Arc<Mutex<SceneManager>>,
    script_engine: Arc<Mutex<ScriptEngine>>,
    running: bool,
}

impl Engine {
    /// Create a new engine instance
    pub fn new() -> Self {
        Self {
            config: GameConfig::default(),
            resource_manager: Arc::new(Mutex::new(ResourceManager::new())),
            render_engine: Arc::new(Mutex::new(RenderEngine::new())),
            scene_manager: Arc::new(Mutex::new(SceneManager::new())),
            script_engine: Arc::new(Mutex::new(ScriptEngine::new())),
            running: false,
        }
    }

    /// Initialize the engine with the given configuration
    pub fn initialize(&mut self, config: GameConfig) -> Result<()> {
        log::info!("Initializing Visual Novel Engine...");
        
        self.config = config;

        // Initialize resource manager
        {
            let mut rm = self.resource_manager.lock();
            rm.initialize()?;
        }

        // Initialize render engine
        {
            let mut re = self.render_engine.lock();
            re.initialize(
                self.config.width,
                self.config.height,
                &self.config.title,
                self.config.vsync,
            )?;
        }

        // Initialize scene manager
        {
            let mut sm = self.scene_manager.lock();
            sm.initialize()?;
        }

        // Initialize script engine
        {
            let mut se = self.script_engine.lock();
            se.initialize()?;
        }

        log::info!("Engine initialized successfully");
        Ok(())
    }

    /// Shutdown the engine
    pub fn shutdown(&mut self) {
        log::info!("Shutting down engine...");
        self.running = false;
        
        // Shutdown subsystems in reverse order
        self.script_engine.lock().shutdown();
        self.scene_manager.lock().shutdown();
        self.render_engine.lock().shutdown();
        self.resource_manager.lock().shutdown();
        
        log::info!("Engine shutdown complete");
    }

    /// Run the main game loop
    pub fn run(&mut self) -> Result<()> {
        if self.running {
            return Ok(());
        }

        self.running = true;
        log::info!("Starting main game loop");

        // Main loop would go here - simplified for this example
        while self.running {
            self.update(1.0 / 60.0)?;
            self.render()?;
        }

        Ok(())
    }

    /// Stop the engine
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Update all subsystems
    fn update(&mut self, delta_time: f32) -> Result<()> {
        self.scene_manager.lock().update(delta_time);
        Ok(())
    }

    /// Render the current frame
    fn render(&mut self) -> Result<()> {
        self.render_engine.lock().clear();
        self.scene_manager.lock().render();
        self.render_engine.lock().present()?;
        Ok(())
    }

    // Getters for subsystems
    pub fn resource_manager(&self) -> Arc<Mutex<ResourceManager>> {
        Arc::clone(&self.resource_manager)
    }

    pub fn render_engine(&self) -> Arc<Mutex<RenderEngine>> {
        Arc::clone(&self.render_engine)
    }

    pub fn scene_manager(&self) -> Arc<Mutex<SceneManager>> {
        Arc::clone(&self.scene_manager)
    }

    pub fn script_engine(&self) -> Arc<Mutex<ScriptEngine>> {
        Arc::clone(&self.script_engine)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if self.running {
            self.shutdown();
        }
    }
}
