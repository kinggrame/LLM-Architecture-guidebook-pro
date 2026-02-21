#pragma once

#include <SFML/Graphics.hpp>
#include <memory>
#include <vector>
#include <functional>

namespace VNE {

class ResourceManager;
class Texture;
class Font;

// UI Component base class
class UIComponent {
public:
    UIComponent() = default;
    virtual ~UIComponent() = default;
    
    virtual void update(float deltaTime) {}
    virtual void render(sf::RenderWindow& window) = 0;
    
    void setPosition(float x, float y) { m_position = sf::Vector2f(x, y); }
    void setPosition(const sf::Vector2f& pos) { m_position = pos; }
    sf::Vector2f getPosition() const { return m_position; }
    
    void setVisible(bool visible) { m_visible = visible; }
    bool isVisible() const { return m_visible; }
    
protected:
    sf::Vector2f m_position;
    bool m_visible = true;
};

// Sprite component for characters and backgrounds
class SpriteComponent : public UIComponent {
public:
    SpriteComponent();
    explicit SpriteComponent(std::shared_ptr<Texture> texture);
    
    void setTexture(std::shared_ptr<Texture> texture);
    void setColor(const sf::Color& color) { m_sprite.setColor(color); }
    void setScale(float x, float y) { m_sprite.setScale(x, y); }
    void setAlpha(uint8_t alpha);
    
    sf::FloatRect getBounds() const { return m_sprite.getGlobalBounds(); }
    
    void update(float deltaTime) override;
    void render(sf::RenderWindow& window) override;
    
private:
    sf::Sprite m_sprite;
    std::shared_ptr<Texture> m_texture;
};

// Text box for dialogue
class TextBox : public UIComponent {
public:
    TextBox();
    
    void setFont(std::shared_ptr<Font> font);
    void setText(const std::string& text);
    void setCharacterSize(unsigned int size) { m_text.setCharacterSize(size); }
    void setTextColor(const sf::Color& color) { m_text.setFillColor(color); }
    void setBoxColor(const sf::Color& color) { m_background.setFillColor(color); }
    void setSize(float width, float height);
    
    void update(float deltaTime) override;
    void render(sf::RenderWindow& window) override;
    
private:
    void wrapText();
    
    sf::RectangleShape m_background;
    sf::Text m_text;
    sf::Text m_speakerName;
    std::shared_ptr<Font> m_font;
    std::string m_fullText;
    std::string m_displayedText;
    float m_typingTimer = 0.0f;
    float m_typingSpeed = 0.05f;
    size_t m_currentChar = 0;
    bool m_typingComplete = false;
};

// Choice button for branching
class ChoiceButton : public UIComponent {
public:
    using Callback = std::function<void()>;
    
    ChoiceButton();
    
    void setFont(std::shared_ptr<Font> font);
    void setText(const std::string& text);
    void setSize(float width, float height);
    void setCallback(Callback callback) { m_callback = callback; }
    
    bool contains(float x, float y) const;
    void onClick();
    
    void update(float deltaTime) override;
    void render(sf::RenderWindow& window) override;
    
private:
    sf::RectangleShape m_background;
    sf::Text m_text;
    std::shared_ptr<Font> m_font;
    Callback m_callback;
    bool m_hovered = false;
};

// Main render engine
class RenderEngine {
public:
    RenderEngine();
    ~RenderEngine();
    
    bool initialize(unsigned int width, unsigned int height, 
                    const std::string& title, bool vsync);
    void shutdown();
    
    void clear(const sf::Color& color = sf::Color::Black);
    void display();
    bool isOpen() const { return m_window.isOpen(); }
    
    sf::RenderWindow& getWindow() { return m_window; }
    
    // Component management
    void addComponent(std::shared_ptr<UIComponent> component);
    void removeComponent(std::shared_ptr<UIComponent> component);
    void clearComponents();
    
    // Rendering
    void renderAll();
    
    // View management
    void setView(const sf::View& view) { m_window.setView(view); }
    sf::View getDefaultView() const { return m_window.getDefaultView(); }

private:
    sf::RenderWindow m_window;
    std::vector<std::shared_ptr<UIComponent>> m_components;
    bool m_initialized = false;
};

} // namespace VNE
