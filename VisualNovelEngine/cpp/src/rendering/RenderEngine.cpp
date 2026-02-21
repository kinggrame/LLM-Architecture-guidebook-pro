#include "rendering/RenderEngine.h"
#include "resources/ResourceManager.h"
#include <algorithm>
#include <iostream>

namespace VNE {

// SpriteComponent implementation
SpriteComponent::SpriteComponent() = default;

SpriteComponent::SpriteComponent(std::shared_ptr<Texture> texture) {
    setTexture(texture);
}

void SpriteComponent::setTexture(std::shared_ptr<Texture> texture) {
    m_texture = texture;
    if (texture && texture->isLoaded()) {
        m_sprite.setTexture(*texture->get());
    }
}

void SpriteComponent::setAlpha(uint8_t alpha) {
    sf::Color color = m_sprite.getColor();
    color.a = alpha;
    m_sprite.setColor(color);
}

void SpriteComponent::update(float deltaTime) {
    // Update sprite position
    m_sprite.setPosition(m_position);
}

void SpriteComponent::render(sf::RenderWindow& window) {
    if (m_visible && m_texture && m_texture->isLoaded()) {
        window.draw(m_sprite);
    }
}

// TextBox implementation
TextBox::TextBox() {
    m_background.setFillColor(sf::Color(0, 0, 0, 200));
    m_background.setOutlineColor(sf::Color(255, 255, 255, 255));
    m_background.setOutlineThickness(2.0f);
    
    m_text.setFillColor(sf::Color::White);
    m_speakerName.setFillColor(sf::Color::Yellow);
    m_speakerName.setCharacterSize(24);
}

void TextBox::setFont(std::shared_ptr<Font> font) {
    m_font = font;
    if (font && font->isLoaded()) {
        m_text.setFont(*font->get());
        m_speakerName.setFont(*font->get());
    }
}

void TextBox::setText(const std::string& text) {
    m_fullText = text;
    m_displayedText.clear();
    m_currentChar = 0;
    m_typingTimer = 0.0f;
    m_typingComplete = false;
}

void TextBox::setSize(float width, float height) {
    m_background.setSize(sf::Vector2f(width, height));
}

void TextBox::update(float deltaTime) {
    UIComponent::update(deltaTime);
    
    if (!m_typingComplete && m_currentChar < m_fullText.length()) {
        m_typingTimer += deltaTime;
        
        while (m_typingTimer >= m_typingSpeed && m_currentChar < m_fullText.length()) {
            m_typingTimer -= m_typingSpeed;
            m_displayedText += m_fullText[m_currentChar];
            m_currentChar++;
        }
        
        if (m_currentChar >= m_fullText.length()) {
            m_typingComplete = true;
        }
        
        m_text.setString(m_displayedText);
    }
    
    // Update positions
    m_background.setPosition(m_position);
    m_text.setPosition(m_position.x + 20, m_position.y + 50);
    m_speakerName.setPosition(m_position.x + 20, m_position.y + 10);
}

void TextBox::render(sf::RenderWindow& window) {
    if (m_visible) {
        window.draw(m_background);
        window.draw(m_speakerName);
        window.draw(m_text);
    }
}

void TextBox::wrapText() {
    // Simple text wrapping logic could be implemented here
}

// ChoiceButton implementation
ChoiceButton::ChoiceButton() {
    m_background.setFillColor(sf::Color(50, 50, 50, 200));
    m_background.setOutlineColor(sf::Color(255, 255, 255, 100));
    m_background.setOutlineThickness(2.0f);
    m_text.setFillColor(sf::Color::White);
}

void ChoiceButton::setFont(std::shared_ptr<Font> font) {
    m_font = font;
    if (font && font->isLoaded()) {
        m_text.setFont(*font->get());
    }
}

void ChoiceButton::setText(const std::string& text) {
    m_text.setString(text);
}

void ChoiceButton::setSize(float width, float height) {
    m_background.setSize(sf::Vector2f(width, height));
}

bool ChoiceButton::contains(float x, float y) const {
    return m_background.getGlobalBounds().contains(x, y);
}

void ChoiceButton::onClick() {
    if (m_callback) {
        m_callback();
    }
}

void ChoiceButton::update(float deltaTime) {
    UIComponent::update(deltaTime);
    
    // Update colors based on hover state
    if (m_hovered) {
        m_background.setFillColor(sf::Color(100, 100, 100, 200));
        m_background.setOutlineColor(sf::Color(255, 255, 255, 255));
    } else {
        m_background.setFillColor(sf::Color(50, 50, 50, 200));
        m_background.setOutlineColor(sf::Color(255, 255, 255, 100));
    }
    
    m_background.setPosition(m_position);
    
    // Center text
    sf::FloatRect textBounds = m_text.getLocalBounds();
    sf::FloatRect bgBounds = m_background.getLocalBounds();
    m_text.setPosition(
        m_position.x + (bgBounds.width - textBounds.width) / 2.0f,
        m_position.y + (bgBounds.height - textBounds.height) / 2.0f - 5.0f
    );
}

void ChoiceButton::render(sf::RenderWindow& window) {
    if (m_visible) {
        window.draw(m_background);
        window.draw(m_text);
    }
}

// RenderEngine implementation
RenderEngine::RenderEngine() = default;

RenderEngine::~RenderEngine() {
    shutdown();
}

bool RenderEngine::initialize(unsigned int width, unsigned int height, 
                               const std::string& title, bool vsync) {
    m_window.create(sf::VideoMode(width, height), title, sf::Style::Close);
    m_window.setVerticalSyncEnabled(vsync);
    
    m_initialized = true;
    std::cout << "RenderEngine initialized: " << width << "x" << height << std::endl;
    return true;
}

void RenderEngine::shutdown() {
    if (m_window.isOpen()) {
        m_window.close();
    }
    m_initialized = false;
}

void RenderEngine::clear(const sf::Color& color) {
    m_window.clear(color);
}

void RenderEngine::display() {
    m_window.display();
}

void RenderEngine::addComponent(std::shared_ptr<UIComponent> component) {
    m_components.push_back(component);
}

void RenderEngine::removeComponent(std::shared_ptr<UIComponent> component) {
    auto it = std::find(m_components.begin(), m_components.end(), component);
    if (it != m_components.end()) {
        m_components.erase(it);
    }
}

void RenderEngine::clearComponents() {
    m_components.clear();
}

void RenderEngine::renderAll() {
    for (auto& component : m_components) {
        if (component->isVisible()) {
            component->render(m_window);
        }
    }
}

} // namespace VNE
