mod core;
mod rendering;
mod resources;
mod scene;
mod script;

use crate::core::config::GameConfig;
use crate::core::engine::Engine;
use crate::resources::ResourceManager;
use crate::scene::scene::VisualNovelScene;
use crate::scene::SceneManager;
use crate::script::command::CommandType;
use crate::script::ScriptEngine;
use anyhow::Result;
use std::sync::Arc;
use parking_lot::Mutex;

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    log::info!("============================================");
    log::info!("  Visual Novel Engine - Rust Version");
    log::info!("============================================");

    // Create engine
    let mut engine = Engine::new();

    // Configure game
    let config = GameConfig {
        title: "Visual Novel Engine - Rust Demo".to_string(),
        width: 1280,
        height: 720,
        vsync: true,
        ..Default::default()
    };

    // Initialize engine
    engine.initialize(config)?;

    // Get subsystems
    let resource_manager = engine.resource_manager();
    let script_engine = engine.script_engine();
    let scene_manager = engine.scene_manager();

    // Setup script command handlers
    {
        let mut se = script_engine.lock();
        
        // Dialogue handler
        se.set_command_handler(CommandType::Dialogue, |cmd| {
            if cmd.params.len() >= 2 {
                println!("{}: {}", cmd.params[0], cmd.params[1]);
            }
        });

        // Background handler
        se.set_command_handler(CommandType::Background, |cmd| {
            if let Some(path) = cmd.params.first() {
                println!("Changing background to: {}", path);
            }
        });

        // Character handler
        se.set_command_handler(CommandType::Character, |cmd| {
            if cmd.params.len() >= 2 {
                println!(
                    "Showing character: {} with image: {}",
                    cmd.params[0], cmd.params[1]
                );
            }
        });

        // Music handler
        se.set_command_handler(CommandType::Music, |cmd| {
            if let Some(path) = cmd.params.first() {
                println!("Playing music: {}", path);
            }
        });

        // Sound handler
        se.set_command_handler(CommandType::Sound, |cmd| {
            if let Some(path) = cmd.params.first() {
                println!("Playing sound: {}", path);
            }
        });
    }

    // Create and register scenes
    {
        let mut sm = scene_manager.lock();
        let main_scene = VisualNovelScene::new("main");
        sm.register_scene("main", main_scene);
    }

    // Load script and start
    {
        let mut se = script_engine.lock();
        let script_path = "../assets/scripts/demo_script.txt";
        match se.load_script(script_path) {
            Ok(_) => {
                log::info!("Script loaded successfully");
                se.start();
            }
            Err(e) => {
                log::error!("Failed to load script: {}", e);
            }
        }
    }

    // Change to main scene
    {
        let mut sm = scene_manager.lock();
        sm.change_scene("main");
    }

    // Demo: Process some script commands
    println!("\n========================================");
    println!("Starting Visual Novel Engine Demo");
    println!("Press SPACE, ENTER, or CLICK to advance");
    println!("========================================\n");

    // Simulate game loop (simplified)
    let mut step = 0;
    loop {
        {
            let se = script_engine.lock();
            if !se.is_running() {
                break;
            }

            if se.is_waiting_for_input() {
                // In a real game, this would wait for user input
                // For demo, we auto-advance
                drop(se);
                let mut se = script_engine.lock();
                se.next();
            }
        }

        step += 1;
        if step > 50 {
            // Safety break for demo
            break;
        }

        // Small delay to simulate frame rate
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("\n========================================");
    println!("Engine stopped. Goodbye!");
    println!("========================================");

    Ok(())
}
