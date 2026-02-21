#include "script/ScriptEngine.h"
#include <fstream>
#include <sstream>
#include <iostream>
#include <algorithm>
#include <cctype>

namespace VNE {

ScriptEngine::ScriptEngine() = default;

ScriptEngine::~ScriptEngine() {
    shutdown();
}

bool ScriptEngine::initialize() {
    std::cout << "ScriptEngine initialized" << std::endl;
    return true;
}

void ScriptEngine::shutdown() {
    stop();
    m_commands.clear();
    m_labels.clear();
    m_handlers.clear();
}

bool ScriptEngine::loadScript(const std::string& path) {
    std::ifstream file(path);
    if (!file.is_open()) {
        std::cerr << "Failed to open script: " << path << std::endl;
        return false;
    }
    
    std::stringstream buffer;
    buffer << file.rdbuf();
    file.close();
    
    return parseScript(buffer.str());
}

bool ScriptEngine::parseScript(const std::string& content) {
    m_commands.clear();
    m_labels.clear();
    m_currentIndex = 0;
    
    std::istringstream stream(content);
    std::string line;
    size_t lineNumber = 0;
    
    while (std::getline(stream, line)) {
        lineNumber++;
        
        // Trim whitespace
        auto start = line.find_first_not_of(" \t\r\n");
        if (start == std::string::npos) continue;
        auto end = line.find_last_not_of(" \t\r\n");
        line = line.substr(start, end - start + 1);
        
        // Skip comments and empty lines
        if (line.empty() || line[0] == '#' || line[0] == '/') {
            continue;
        }
        
        // Parse commands
        if (line[0] == '@') {
            // Command
            size_t spacePos = line.find(' ');
            std::string cmd = line.substr(1, spacePos - 1);
            std::string params = (spacePos != std::string::npos) ? line.substr(spacePos + 1) : "";
            
            Command command;
            
            if (cmd == "bg" || cmd == "background") {
                command.type = CommandType::Background;
            } else if (cmd == "char" || cmd == "character") {
                command.type = CommandType::Character;
            } else if (cmd == "music" || cmd == "bgm") {
                command.type = CommandType::Music;
            } else if (cmd == "sound" || cmd == "sfx") {
                command.type = CommandType::Sound;
            } else if (cmd == "wait") {
                command.type = CommandType::Wait;
            } else if (cmd == "jump") {
                command.type = CommandType::Jump;
            } else if (cmd == "end") {
                command.type = CommandType::End;
            } else {
                command.type = CommandType::None;
            }
            
            // Parse parameters
            if (!params.empty()) {
                std::istringstream paramStream(params);
                std::string param;
                while (paramStream >> param) {
                    command.params.push_back(param);
                }
            }
            
            m_commands.push_back(command);
        } else if (line[0] == '*') {
            // Label
            std::string label = line.substr(1);
            m_labels[label] = m_commands.size();
            
            Command command;
            command.type = CommandType::Label;
            command.params.push_back(label);
            m_commands.push_back(command);
        } else if (line.find(":") != std::string::npos) {
            // Dialogue
            size_t colonPos = line.find(':');
            std::string speaker = line.substr(0, colonPos);
            std::string text = line.substr(colonPos + 1);
            
            // Trim
            speaker.erase(0, speaker.find_first_not_of(" \t"));
            speaker.erase(speaker.find_last_not_of(" \t") + 1);
            text.erase(0, text.find_first_not_of(" \t"));
            text.erase(text.find_last_not_of(" \t") + 1);
            
            Command command;
            command.type = CommandType::Dialogue;
            command.params.push_back(speaker);
            command.params.push_back(text);
            m_commands.push_back(command);
        } else if (line[0] == '[' && line.back() == ']') {
            // Choice
            Command command;
            command.type = CommandType::Choice;
            // Parse choices (simplified)
            m_commands.push_back(command);
        }
    }
    
    std::cout << "Loaded script with " << m_commands.size() << " commands" << std::endl;
    return true;
}

void ScriptEngine::start() {
    m_running = true;
    m_paused = false;
    m_currentIndex = 0;
    executeCurrentCommand();
}

void ScriptEngine::stop() {
    m_running = false;
    m_paused = false;
}

void ScriptEngine::pause() {
    m_paused = true;
}

void ScriptEngine::resume() {
    m_paused = false;
}

void ScriptEngine::next() {
    if (m_waitingForInput) {
        m_waitingForInput = false;
        m_currentIndex++;
        if (m_currentIndex < m_commands.size()) {
            executeCurrentCommand();
        } else {
            m_running = false;
        }
    }
}

void ScriptEngine::executeCurrentCommand() {
    if (m_currentIndex >= m_commands.size()) {
        m_running = false;
        return;
    }
    
    const Command& command = m_commands[m_currentIndex];
    
    auto it = m_handlers.find(command.type);
    if (it != m_handlers.end()) {
        it->second(command);
    }
    
    // Handle automatic commands
    if (command.type == CommandType::Dialogue || 
        command.type == CommandType::Choice) {
        m_waitingForInput = true;
    } else if (command.type == CommandType::End) {
        m_running = false;
    } else if (command.type == CommandType::Label) {
        // Skip labels automatically
        m_currentIndex++;
        executeCurrentCommand();
    } else {
        // Auto-advance for other commands
        m_currentIndex++;
        executeCurrentCommand();
    }
}

void ScriptEngine::setCommandHandler(CommandType type, CommandHandler handler) {
    m_handlers[type] = handler;
}

void ScriptEngine::clearCommandHandler(CommandType type) {
    m_handlers.erase(type);
}

const Command* ScriptEngine::getCurrentCommand() const {
    if (m_currentIndex < m_commands.size()) {
        return &m_commands[m_currentIndex];
    }
    return nullptr;
}

bool ScriptEngine::jumpToLabel(const std::string& label) {
    auto it = m_labels.find(label);
    if (it != m_labels.end()) {
        m_currentIndex = it->second + 1; // Jump after the label
        executeCurrentCommand();
        return true;
    }
    return false;
}

} // namespace VNE
