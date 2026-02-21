#pragma once

#include <string>
#include <memory>
#include <vector>
#include <map>
#include <functional>

namespace VNE {

// Forward declarations
class Engine;
class RenderEngine;
class ResourceManager;
class ScriptEngine;

// Scene base class
class Scene {
public:
    Scene(const std::string& name);
    virtual ~Scene() = default;
    
    virtual void onEnter() {}
    virtual void onExit() {}
    virtual void onUpdate(float deltaTime) {}
    virtual void onRender() {}
    
    const std::string& getName() const { return m_name; }
    
protected:
    std::string m_name;
};

// Visual Novel Scene - Main game scene
class VisualNovelScene : public Scene {
public:
    VisualNovelScene(const std::string& name);
    
    void onEnter() override;
    void onExit() override;
    void onUpdate(float deltaTime) override;
    void onRender() override;
    
    void loadScript(const std::string& scriptPath);
    void nextLine();
    void makeChoice(int choiceIndex);

private:
    std::string m_currentScriptPath;
    bool m_waitingForInput = false;
    bool m_showingChoices = false;
};

// Scene Manager
class SceneManager {
public:
    SceneManager();
    ~SceneManager();
    
    bool initialize();
    void shutdown();
    
    // Scene registration
    void registerScene(const std::string& name, std::shared_ptr<Scene> scene);
    
    // Scene transition
    void changeScene(const std::string& name);
    void pushScene(const std::string& name);
    void popScene();
    
    // Update and render
    void update(float deltaTime);
    void render();
    
    // Get current scene
    Scene* getCurrentScene() const;
    
private:
    std::map<std::string, std::shared_ptr<Scene>> m_scenes;
    std::vector<std::shared_ptr<Scene>> m_sceneStack;
};

} // namespace VNE
