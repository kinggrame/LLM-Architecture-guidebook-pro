#include "core/Engine.h"
#include "core/EventManager.h"
#include "scene/SceneManager.h"
#include "rendering/RenderEngine.h"
#include "resources/ResourceManager.h"
#include "script/ScriptEngine.h"
#include <iostream>

using namespace VNE;

int main() {
    // Create engine
    Engine engine;
    
    // Configure game
    GameConfig config;
    config.title = "Visual Novel Engine - Demo";
    config.width = 1280;
    config.height = 720;
    config.vsync = true;
    
    // Initialize engine
    if (!engine.initialize(config)) {
        std::cerr << "Failed to initialize engine" << std::endl;
        return -1;
    }
    
    // Setup event handlers
    EventManager* eventManager = engine.getEventManager();
    ScriptEngine* scriptEngine = engine.getScriptEngine();
    RenderEngine* renderEngine = engine.getRenderEngine();
    ResourceManager* resourceManager = engine.getResourceManager();
    SceneManager* sceneManager = engine.getSceneManager();
    
    // Handle advance text events
    eventManager->addListener(EventType::AdvanceText, 
        [&scriptEngine](const Event& event) {
            if (scriptEngine->isWaitingForInput()) {
                scriptEngine->next();
            }
        });
    
    // Setup script command handlers
    scriptEngine->setCommandHandler(CommandType::Dialogue, 
        [&renderEngine, &resourceManager](const Command& cmd) {
            if (cmd.params.size() >= 2) {
                std::cout << cmd.params[0] << ": " << cmd.params[1] << std::endl;
                // Create text box to display dialogue
                // (In a full implementation, this would update the UI)
            }
        });
    
    scriptEngine->setCommandHandler(CommandType::Background,
        [&renderEngine, &resourceManager](const Command& cmd) {
            if (!cmd.params.empty()) {
                std::cout << "Changing background to: " << cmd.params[0] << std::endl;
                // Load and display background
                auto texture = resourceManager->loadTexture("images/" + cmd.params[0]);
                if (texture) {
                    // Create background sprite
                    // (In a full implementation, this would update the scene)
                }
            }
        });
    
    scriptEngine->setCommandHandler(CommandType::Character,
        [&renderEngine, &resourceManager](const Command& cmd) {
            if (cmd.params.size() >= 2) {
                std::cout << "Showing character: " << cmd.params[0] 
                          << " with image: " << cmd.params[1] << std::endl;
                // Load and display character sprite
            }
        });
    
    scriptEngine->setCommandHandler(CommandType::Music,
        [&resourceManager](const Command& cmd) {
            if (!cmd.params.empty()) {
                std::cout << "Playing music: " << cmd.params[0] << std::endl;
                auto audio = resourceManager->loadAudio("audio/" + cmd.params[0]);
                if (audio) {
                    audio->play();
                }
            }
        });
    
    scriptEngine->setCommandHandler(CommandType::Sound,
        [&resourceManager](const Command& cmd) {
            if (!cmd.params.empty()) {
                std::cout << "Playing sound: " << cmd.params[0] << std::endl;
                auto audio = resourceManager->loadAudio("audio/" + cmd.params[0]);
                if (audio) {
                    audio->play();
                }
            }
        });
    
    // Create and register scenes
    auto mainScene = std::make_shared<VisualNovelScene>("main");
    sceneManager->registerScene("main", mainScene);
    
    // Load and start script
    mainScene->loadScript("assets/scripts/demo_script.txt");
    
    // Change to main scene
    sceneManager->changeScene("main");
    
    // Run the engine
    std::cout << "Starting Visual Novel Engine..." << std::endl;
    std::cout << "Press SPACE, ENTER, or CLICK to advance text" << std::endl;
    std::cout << "=========================================" << std::endl;
    
    engine.run();
    
    std::cout << "Engine stopped. Goodbye!" << std::endl;
    return 0;
}
