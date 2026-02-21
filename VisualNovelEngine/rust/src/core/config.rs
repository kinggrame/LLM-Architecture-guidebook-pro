/// Game configuration
#[derive(Debug, Clone)]
pub struct GameConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub target_fps: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            title: "Visual Novel".to_string(),
            width: 1280,
            height: 720,
            fullscreen: false,
            vsync: true,
            target_fps: 60,
        }
    }
}
