# Visual Novel Engine - Rust Version

## Architecture

### Core Components

1. **Scene Manager** - Manages game scenes and state transitions
2. **Rendering System** - Displays backgrounds, characters, UI elements
3. **Script Engine** - Parses and executes visual novel scripts
4. **Asset Manager** - Handles loading of images, audio, fonts
5. **Event System** - Processes user input and game events

### Dependencies
- wgpu (Graphics rendering)
- winit (Window management)
- rodio (Audio playback)
- serde (Script serialization)

## Build Instructions
```bash
cargo build --release
```
