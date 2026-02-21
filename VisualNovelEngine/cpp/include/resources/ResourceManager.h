#pragma once

#include <SFML/Graphics.hpp>
#include <SFML/Audio.hpp>
#include <string>
#include <memory>
#include <map>

namespace VNE {

// Texture wrapper with reference counting
class Texture {
public:
    Texture() = default;
    explicit Texture(const std::string& path);
    
    bool load(const std::string& path);
    bool isLoaded() const { return m_loaded; }
    
    sf::Texture* get() { return &m_texture; }
    const sf::Texture* get() const { return &m_texture; }
    
private:
    sf::Texture m_texture;
    std::string m_path;
    bool m_loaded = false;
};

// Audio wrapper
class AudioClip {
public:
    AudioClip() = default;
    explicit AudioClip(const std::string& path);
    
    bool load(const std::string& path);
    bool isLoaded() const { return m_loaded; }
    
    void play();
    void stop();
    void pause();
    void setLoop(bool loop);
    void setVolume(float volume);
    
private:
    sf::SoundBuffer m_buffer;
    sf::Sound m_sound;
    std::string m_path;
    bool m_loaded = false;
};

// Font wrapper
class Font {
public:
    Font() = default;
    explicit Font(const std::string& path);
    
    bool load(const std::string& path);
    bool isLoaded() const { return m_loaded; }
    
    sf::Font* get() { return &m_font; }
    const sf::Font* get() const { return &m_font; }
    
private:
    sf::Font m_font;
    std::string m_path;
    bool m_loaded = false;
};

// Resource Manager - Handles loading and caching of resources
class ResourceManager {
public:
    ResourceManager();
    ~ResourceManager();
    
    bool initialize();
    void shutdown();
    
    // Texture management
    std::shared_ptr<Texture> loadTexture(const std::string& path);
    void unloadTexture(const std::string& path);
    std::shared_ptr<Texture> getTexture(const std::string& path);
    
    // Audio management
    std::shared_ptr<AudioClip> loadAudio(const std::string& path);
    void unloadAudio(const std::string& path);
    std::shared_ptr<AudioClip> getAudio(const std::string& path);
    
    // Font management
    std::shared_ptr<Font> loadFont(const std::string& path);
    void unloadFont(const std::string& path);
    std::shared_ptr<Font> getFont(const std::string& path);
    
    // Clear all resources
    void clear();
    
    // Set base path for assets
    void setBasePath(const std::string& path) { m_basePath = path; }
    
private:
    std::string m_basePath = "assets/";
    
    std::map<std::string, std::weak_ptr<Texture>> m_textures;
    std::map<std::string, std::weak_ptr<AudioClip>> m_audio;
    std::map<std::string, std::weak_ptr<Font>> m_fonts;
};

} // namespace VNE
