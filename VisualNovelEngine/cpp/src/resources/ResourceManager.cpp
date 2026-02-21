#include "resources/ResourceManager.h"
#include <iostream>

namespace VNE {

// Texture implementation
Texture::Texture(const std::string& path) {
    load(path);
}

bool Texture::load(const std::string& path) {
    m_path = path;
    m_loaded = m_texture.loadFromFile(path);
    if (!m_loaded) {
        std::cerr << "Failed to load texture: " << path << std::endl;
    }
    return m_loaded;
}

// AudioClip implementation
AudioClip::AudioClip(const std::string& path) {
    load(path);
}

bool AudioClip::load(const std::string& path) {
    m_path = path;
    m_loaded = m_buffer.loadFromFile(path);
    if (m_loaded) {
        m_sound.setBuffer(m_buffer);
    } else {
        std::cerr << "Failed to load audio: " << path << std::endl;
    }
    return m_loaded;
}

void AudioClip::play() {
    if (m_loaded) {
        m_sound.play();
    }
}

void AudioClip::stop() {
    if (m_loaded) {
        m_sound.stop();
    }
}

void AudioClip::pause() {
    if (m_loaded) {
        m_sound.pause();
    }
}

void AudioClip::setLoop(bool loop) {
    m_sound.setLoop(loop);
}

void AudioClip::setVolume(float volume) {
    m_sound.setVolume(volume * 100.0f);
}

// Font implementation
Font::Font(const std::string& path) {
    load(path);
}

bool Font::load(const std::string& path) {
    m_path = path;
    m_loaded = m_font.loadFromFile(path);
    if (!m_loaded) {
        std::cerr << "Failed to load font: " << path << std::endl;
    }
    return m_loaded;
}

// ResourceManager implementation
ResourceManager::ResourceManager() = default;

ResourceManager::~ResourceManager() {
    shutdown();
}

bool ResourceManager::initialize() {
    std::cout << "ResourceManager initialized" << std::endl;
    return true;
}

void ResourceManager::shutdown() {
    clear();
}

std::shared_ptr<Texture> ResourceManager::loadTexture(const std::string& path) {
    std::string fullPath = m_basePath + path;
    
    // Check if already loaded
    auto it = m_textures.find(path);
    if (it != m_textures.end()) {
        if (auto texture = it->second.lock()) {
            return texture;
        }
    }
    
    // Load new texture
    auto texture = std::make_shared<Texture>(fullPath);
    if (texture->isLoaded()) {
        m_textures[path] = texture;
        return texture;
    }
    
    return nullptr;
}

void ResourceManager::unloadTexture(const std::string& path) {
    m_textures.erase(path);
}

std::shared_ptr<Texture> ResourceManager::getTexture(const std::string& path) {
    auto it = m_textures.find(path);
    if (it != m_textures.end()) {
        return it->second.lock();
    }
    return nullptr;
}

std::shared_ptr<AudioClip> ResourceManager::loadAudio(const std::string& path) {
    std::string fullPath = m_basePath + path;
    
    auto it = m_audio.find(path);
    if (it != m_audio.end()) {
        if (auto audio = it->second.lock()) {
            return audio;
        }
    }
    
    auto audio = std::make_shared<AudioClip>(fullPath);
    if (audio->isLoaded()) {
        m_audio[path] = audio;
        return audio;
    }
    
    return nullptr;
}

void ResourceManager::unloadAudio(const std::string& path) {
    m_audio.erase(path);
}

std::shared_ptr<AudioClip> ResourceManager::getAudio(const std::string& path) {
    auto it = m_audio.find(path);
    if (it != m_audio.end()) {
        return it->second.lock();
    }
    return nullptr;
}

std::shared_ptr<Font> ResourceManager::loadFont(const std::string& path) {
    std::string fullPath = m_basePath + path;
    
    auto it = m_fonts.find(path);
    if (it != m_fonts.end()) {
        if (auto font = it->second.lock()) {
            return font;
        }
    }
    
    auto font = std::make_shared<Font>(fullPath);
    if (font->isLoaded()) {
        m_fonts[path] = font;
        return font;
    }
    
    return nullptr;
}

void ResourceManager::unloadFont(const std::string& path) {
    m_fonts.erase(path);
}

std::shared_ptr<Font> ResourceManager::getFont(const std::string& path) {
    auto it = m_fonts.find(path);
    if (it != m_fonts.end()) {
        return it->second.lock();
    }
    return nullptr;
}

void ResourceManager::clear() {
    m_textures.clear();
    m_audio.clear();
    m_fonts.clear();
}

} // namespace VNE
