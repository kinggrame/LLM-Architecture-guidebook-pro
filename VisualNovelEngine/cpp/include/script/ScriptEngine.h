#pragma once

#include <string>
#include <vector>
#include <map>
#include <memory>
#include <functional>

namespace VNE {

// Script command types
enum class CommandType {
    Dialogue,
    Choice,
    Background,
    Character,
    Music,
    Sound,
    Transition,
    Wait,
    Label,
    Jump,
    End
};

// Base command structure
struct Command {
    CommandType type;
    std::vector<std::string> params;
};

// Dialogue command
struct DialogueCommand : public Command {
    std::string speaker;
    std::string text;
};

// Choice option
struct ChoiceOption {
    std::string text;
    std::string targetLabel;
};

// Choice command
struct ChoiceCommand : public Command {
    std::vector<ChoiceOption> options;
};

// Background command
struct BackgroundCommand : public Command {
    std::string imagePath;
    std::string transitionType;
    float duration;
};

// Character command
struct CharacterCommand : public Command {
    std::string characterId;
    std::string imagePath;
    std::string position;
    std::string emotion;
};

// Script Engine - Parses and executes visual novel scripts
class ScriptEngine {
public:
    using CommandHandler = std::function<void(const Command&)>;
    
    ScriptEngine();
    ~ScriptEngine();
    
    bool initialize();
    void shutdown();
    
    // Load and parse script
    bool loadScript(const std::string& path);
    
    // Script execution
    void start();
    void stop();
    void pause();
    void resume();
    void next();
    
    // Current state
    bool isRunning() const { return m_running; }
    bool isPaused() const { return m_paused; }
    bool isWaitingForInput() const { return m_waitingForInput; }
    
    // Command handlers
    void setCommandHandler(CommandType type, CommandHandler handler);
    void clearCommandHandler(CommandType type);
    
    // Get current command
    const Command* getCurrentCommand() const;
    
    // Jump to label
    bool jumpToLabel(const std::string& label);

private:
    void executeCurrentCommand();
    bool parseScript(const std::string& content);
    
    std::vector<Command> m_commands;
    std::map<std::string, size_t> m_labels;
    std::map<CommandType, CommandHandler> m_handlers;
    
    size_t m_currentIndex = 0;
    bool m_running = false;
    bool m_paused = false;
    bool m_waitingForInput = false;
};

} // namespace VNE
