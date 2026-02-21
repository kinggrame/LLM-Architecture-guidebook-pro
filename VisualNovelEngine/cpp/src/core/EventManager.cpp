#include "core/EventManager.h"
#include <iostream>

namespace VNE {

EventManager::EventManager() = default;

EventManager::~EventManager() {
    shutdown();
}

bool EventManager::initialize() {
    std::cout << "EventManager initialized" << std::endl;
    return true;
}

void EventManager::shutdown() {
    m_listeners.clear();
}

void EventManager::processEvents(sf::RenderWindow& window) {
    sf::Event sfEvent;
    
    while (window.pollEvent(sfEvent)) {
        Event event;
        
        switch (sfEvent.type) {
            case sf::Event::Closed:
                event.type = EventType::WindowClose;
                break;
                
            case sf::Event::KeyPressed:
                event.type = EventType::KeyPressed;
                event.keyCode = sfEvent.key.code;
                
                // Advance text on space, enter, or mouse click
                if (sfEvent.key.code == sf::Keyboard::Space || 
                    sfEvent.key.code == sf::Keyboard::Return) {
                    Event advanceEvent;
                    advanceEvent.type = EventType::AdvanceText;
                    dispatchEvent(advanceEvent);
                }
                break;
                
            case sf::Event::KeyReleased:
                event.type = EventType::KeyReleased;
                event.keyCode = sfEvent.key.code;
                break;
                
            case sf::Event::MouseButtonPressed:
                event.type = EventType::MouseButtonPressed;
                event.mouseButton = sfEvent.mouseButton.button;
                event.mouseX = sfEvent.mouseButton.x;
                event.mouseY = sfEvent.mouseButton.y;
                
                // Advance text on mouse click
                if (sfEvent.mouseButton.button == sf::Mouse::Left) {
                    Event advanceEvent;
                    advanceEvent.type = EventType::AdvanceText;
                    advanceEvent.mouseX = sfEvent.mouseButton.x;
                    advanceEvent.mouseY = sfEvent.mouseButton.y;
                    dispatchEvent(advanceEvent);
                }
                break;
                
            case sf::Event::MouseButtonReleased:
                event.type = EventType::MouseButtonReleased;
                event.mouseButton = sfEvent.mouseButton.button;
                event.mouseX = sfEvent.mouseButton.x;
                event.mouseY = sfEvent.mouseButton.y;
                break;
                
            case sf::Event::MouseMoved:
                event.type = EventType::MouseMoved;
                event.mouseX = sfEvent.mouseMove.x;
                event.mouseY = sfEvent.mouseMove.y;
                break;
                
            case sf::Event::TextEntered:
                event.type = EventType::TextEntered;
                event.unicode = sfEvent.text.unicode;
                break;
                
            default:
                event.type = EventType::None;
                break;
        }
        
        if (event.type != EventType::None) {
            dispatchEvent(event);
        }
    }
}

void EventManager::addListener(EventType type, EventListener listener) {
    m_listeners[type].push_back(listener);
}

void EventManager::removeListener(EventType type, EventListener listener) {
    auto it = m_listeners.find(type);
    if (it != m_listeners.end()) {
        auto& listeners = it->second;
        listeners.erase(
            std::remove_if(listeners.begin(), listeners.end(),
                [&listener](const EventListener& l) {
                    return l.target<void(const Event&)>() == 
                           listener.target<void(const Event&)>();
                }),
            listeners.end()
        );
    }
}

void EventManager::clearListeners(EventType type) {
    m_listeners.erase(type);
}

void EventManager::triggerEvent(const Event& event) {
    dispatchEvent(event);
}

void EventManager::dispatchEvent(const Event& event) {
    auto it = m_listeners.find(event.type);
    if (it != m_listeners.end()) {
        for (auto& listener : it->second) {
            listener(event);
        }
    }
}

bool EventManager::isKeyPressed(sf::Keyboard::Key key) const {
    return sf::Keyboard::isKeyPressed(key);
}

bool EventManager::isMouseButtonPressed(sf::Mouse::Button button) const {
    return sf::Mouse::isButtonPressed(button);
}

sf::Vector2i EventManager::getMousePosition(const sf::RenderWindow& window) const {
    return sf::Mouse::getPosition(window);
}

} // namespace VNE
