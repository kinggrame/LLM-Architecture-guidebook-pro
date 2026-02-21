#include "core/Engine.h"
#include "core/EventManager.h"
#include "scene/SceneManager.h"
#include "rendering/RenderEngine.h"
#include "resources/ResourceManager.h"
#include "script/ScriptEngine.h"
#include <SFML/System/Clock.hpp>
#include <iostream>

namespace VNE {

Engine* Engine::s_instance = nullptr;

Engine::Engine() {
    s_instance = this;
}

Engine::~Engine() {
    shutdown();
    s_instance = nullptr;
}

bool Engine::initialize(const GameConfig& config) {
    if (m_initialized) {
        return true;
    }
    
    m_config = config;
    
    // Initialize subsystems
    m_resourceManager = std::make_unique<ResourceManager>();
    if (!m_resourceManager->initialize()) {
        std::cerr << "Failed to initialize ResourceManager" << std::endl;
        return false;
    }
    
    m_renderEngine = std::make_unique<RenderEngine>();
    if (!m_renderEngine->initialize(config.width, config.height, config.title, config.vsync)) {
        std::cerr << "Failed to initialize RenderEngine" << std::endl;
        return false;
    }
    
    m_eventManager = std::make_unique<EventManager>();
    if (!m_eventManager->initialize()) {
        std::cerr << "Failed to initialize EventManager" << std::endl;
        return false;
    }
    
    m_sceneManager = std::make_unique<SceneManager>();
    if (!m_sceneManager->initialize()) {
        std::cerr << "Failed to initialize SceneManager" << std::endl;
        return false;
    }
    
    m_scriptEngine = std::make_unique<ScriptEngine>();
    if (!m_scriptEngine->initialize()) {
        std::cerr << "Failed to initialize ScriptEngine" << std::endl;
        return false;
    }
    
    m_initialized = true;
    std::cout << "Engine initialized successfully" << std::endl;
    return true;
}

void Engine::shutdown() {
    if (!m_initialized) {
        return;
    }
    
    m_running = false;
    
    if (m_scriptEngine) {
        m_scriptEngine->shutdown();
        m_scriptEngine.reset();
    }
    
    if (m_sceneManager) {
        m_sceneManager->shutdown();
        m_sceneManager.reset();
    }
    
    if (m_eventManager) {
        m_eventManager->shutdown();
        m_eventManager.reset();
    }
    
    if (m_renderEngine) {
        m_renderEngine->shutdown();
        m_renderEngine.reset();
    }
    
    if (m_resourceManager) {
        m_resourceManager->shutdown();
        m_resourceManager.reset();
    }
    
    m_initialized = false;
    std::cout << "Engine shutdown complete" << std::endl;
}

void Engine::run() {
    if (!m_initialized) {
        std::cerr << "Engine not initialized" << std::endl;
        return;
    }
    
    m_running = true;
    sf::Clock deltaClock;
    
    while (m_running && m_renderEngine->isOpen()) {
        float deltaTime = deltaClock.restart().asSeconds();
        
        processEvents();
        update(deltaTime);
        render();
    }
}

void Engine::stop() {
    m_running = false;
}

void Engine::processEvents() {
    m_eventManager->processEvents(m_renderEngine->getWindow());
    
    // Check for window close
    if (!m_renderEngine->isOpen()) {
        m_running = false;
    }
}

void Engine::update(float deltaTime) {
    m_sceneManager->update(deltaTime);
}

void Engine::render() {
    m_renderEngine->clear();
    m_sceneManager->render();
    m_renderEngine->display();
}

} // namespace VNE
