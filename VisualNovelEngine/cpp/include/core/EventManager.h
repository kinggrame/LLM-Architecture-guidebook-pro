#pragma once

#include <SFML/Window.hpp>
#include <functional>
#include <map>
#include <vector>

namespace VNE {

// Event types
enum class EventType {
    None = 0,
    WindowClose,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    TextEntered,
    AdvanceText,
    ChoiceSelected
};

// Event structure
struct Event {
    EventType type = EventType::None;
    
    // Keyboard event data
    sf::Keyboard::Key keyCode = sf::Keyboard::Unknown;
    
    // Mouse event data
    sf::Mouse::Button mouseButton = sf::Mouse::Left;
    int mouseX = 0;
    int mouseY = 0;
    
    // Text input
    uint32_t unicode = 0;
    
    // Choice selection
    int choiceIndex = -1;
};

// Event listener
using EventListener = std::function<void(const Event&)>;

// Event Manager - Handles input and game events
class EventManager {
public:
    EventManager();
    ~EventManager();
    
    bool initialize();
    void shutdown();
    
    // Event processing
    void processEvents(sf::RenderWindow& window);
    
    // Event listeners
    void addListener(EventType type, EventListener listener);
    void removeListener(EventType type, EventListener listener);
    void clearListeners(EventType type);
    
    // Trigger event manually
    void triggerEvent(const Event& event);
    
    // Input state
    bool isKeyPressed(sf::Keyboard::Key key) const;
    bool isMouseButtonPressed(sf::Mouse::Button button) const;
    sf::Vector2i getMousePosition(const sf::RenderWindow& window) const;

private:
    void dispatchEvent(const Event& event);
    
    std::map<EventType, std::vector<EventListener>> m_listeners;
};

} // namespace VNE
