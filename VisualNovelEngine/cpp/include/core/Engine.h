#pragma once

#include <string>
#include <memory>
#include <vector>
#include <map>
#include <functional>

namespace VNE {

// Forward declarations
class SceneManager;
class RenderEngine;
class ResourceManager;
class ScriptEngine;
class EventManager;

// Game configuration
struct GameConfig {
    std::string title = "Visual Novel";
    unsigned int width = 1280;
    unsigned int height = 720;
    bool fullscreen = false;
    bool vsync = true;
    unsigned int targetFPS = 60;
};

// Main engine class
class Engine {
public:
    Engine();
    ~Engine();

    bool initialize(const GameConfig& config);
    void shutdown();
    
    void run();
    void stop();
    
    SceneManager* getSceneManager() const { return m_sceneManager.get(); }
    RenderEngine* getRenderEngine() const { return m_renderEngine.get(); }
    ResourceManager* getResourceManager() const { return m_resourceManager.get(); }
    ScriptEngine* getScriptEngine() const { return m_scriptEngine.get(); }
    EventManager* getEventManager() const { return m_eventManager.get(); }
    
    static Engine* getInstance() { return s_instance; }

private:
    void processEvents();
    void update(float deltaTime);
    void render();

    static Engine* s_instance;
    
    std::unique_ptr<SceneManager> m_sceneManager;
    std::unique_ptr<RenderEngine> m_renderEngine;
    std::unique_ptr<ResourceManager> m_resourceManager;
    std::unique_ptr<ScriptEngine> m_scriptEngine;
    std::unique_ptr<EventManager> m_eventManager;
    
    GameConfig m_config;
    bool m_running = false;
    bool m_initialized = false;
};

} // namespace VNE
