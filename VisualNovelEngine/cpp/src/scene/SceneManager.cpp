#include "scene/SceneManager.h"
#include "core/Engine.h"
#include "core/EventManager.h"
#include "rendering/RenderEngine.h"
#include "script/ScriptEngine.h"
#include <iostream>

namespace VNE {

// Scene base class implementation
Scene::Scene(const std::string& name) : m_name(name) {}

// VisualNovelScene implementation
VisualNovelScene::VisualNovelScene(const std::string& name) 
    : Scene(name) {}

void VisualNovelScene::onEnter() {
    std::cout << "Entering scene: " << m_name << std::endl;
}

void VisualNovelScene::onExit() {
    std::cout << "Exiting scene: " << m_name << std::endl;
}

void VisualNovelScene::onUpdate(float deltaTime) {
    // Update scene logic
}

void VisualNovelScene::onRender() {
    // Rendering is handled by RenderEngine components
}

void VisualNovelScene::loadScript(const std::string& scriptPath) {
    m_currentScriptPath = scriptPath;
    
    if (Engine::getInstance()) {
        auto scriptEngine = Engine::getInstance()->getScriptEngine();
        if (scriptEngine) {
            scriptEngine->loadScript(scriptPath);
            scriptEngine->start();
        }
    }
}

void VisualNovelScene::nextLine() {
    if (Engine::getInstance()) {
        auto scriptEngine = Engine::getInstance()->getScriptEngine();
        if (scriptEngine && scriptEngine->isWaitingForInput()) {
            scriptEngine->next();
        }
    }
}

void VisualNovelScene::makeChoice(int choiceIndex) {
    // Handle choice selection
    std::cout << "Choice selected: " << choiceIndex << std::endl;
}

// SceneManager implementation
SceneManager::SceneManager() = default;

SceneManager::~SceneManager() {
    shutdown();
}

bool SceneManager::initialize() {
    std::cout << "SceneManager initialized" << std::endl;
    return true;
}

void SceneManager::shutdown() {
    m_sceneStack.clear();
    m_scenes.clear();
}

void SceneManager::registerScene(const std::string& name, std::shared_ptr<Scene> scene) {
    m_scenes[name] = scene;
}

void SceneManager::changeScene(const std::string& name) {
    auto it = m_scenes.find(name);
    if (it == m_scenes.end()) {
        std::cerr << "Scene not found: " << name << std::endl;
        return;
    }
    
    // Exit current scene
    if (!m_sceneStack.empty()) {
        m_sceneStack.back()->onExit();
        m_sceneStack.clear();
    }
    
    // Enter new scene
    m_sceneStack.push_back(it->second);
    it->second->onEnter();
}

void SceneManager::pushScene(const std::string& name) {
    auto it = m_scenes.find(name);
    if (it == m_scenes.end()) {
        std::cerr << "Scene not found: " << name << std::endl;
        return;
    }
    
    // Exit current scene
    if (!m_sceneStack.empty()) {
        m_sceneStack.back()->onExit();
    }
    
    // Push new scene
    m_sceneStack.push_back(it->second);
    it->second->onEnter();
}

void SceneManager::popScene() {
    if (m_sceneStack.empty()) {
        return;
    }
    
    m_sceneStack.back()->onExit();
    m_sceneStack.pop_back();
    
    // Enter previous scene
    if (!m_sceneStack.empty()) {
        m_sceneStack.back()->onEnter();
    }
}

void SceneManager::update(float deltaTime) {
    if (!m_sceneStack.empty()) {
        m_sceneStack.back()->onUpdate(deltaTime);
    }
}

void SceneManager::render() {
    if (!m_sceneStack.empty()) {
        m_sceneStack.back()->onRender();
    }
    
    // Render all UI components
    if (Engine::getInstance()) {
        Engine::getInstance()->getRenderEngine()->renderAll();
    }
}

Scene* SceneManager::getCurrentScene() const {
    if (m_sceneStack.empty()) {
        return nullptr;
    }
    return m_sceneStack.back().get();
}

} // namespace VNE
