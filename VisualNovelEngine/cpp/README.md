# Visual Novel Engine - C++ Version

## Architecture

### Core Components

1. **Scene Manager** - Manages game scenes and state transitions
2. **Rendering System** - Displays backgrounds, characters, UI elements
3. **Script Engine** - Parses and executes visual novel scripts
4. **Resource Manager** - Handles loading of images, audio, fonts
5. **Event System** - Processes user input and game events

### Dependencies
- SFML 2.6+ (Graphics, Audio, Window)
- CMake 3.16+

## Build Instructions
```bash
mkdir build && cd build
cmake ..
cmake --build .
```
