pub mod command;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use command::{Command, CommandType};

/// Command handler type
pub type CommandHandler = Box<dyn Fn(&Command) + Send>;

/// Script Engine parses and executes visual novel scripts
pub struct ScriptEngine {
    commands: Vec<Command>,
    labels: HashMap<String, usize>,
    handlers: HashMap<CommandType, Vec<CommandHandler>>,
    current_index: usize,
    running: bool,
    paused: bool,
    waiting_for_input: bool,
}

impl ScriptEngine {
    /// Create a new script engine
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            labels: HashMap::new(),
            handlers: HashMap::new(),
            current_index: 0,
            running: false,
            paused: false,
            waiting_for_input: false,
        }
    }

    /// Set a command handler for a specific command type
    pub fn set_command_handler<F>(&mut self, command_type: CommandType, handler: F)
    where
        F: Fn(&Command) + Send + 'static,
    {
        self.handlers
            .entry(command_type)
            .or_default()
            .push(Box::new(handler));
    }

    /// Clear handlers for a specific command type
    pub fn clear_handlers(&mut self, command_type: CommandType) {
        self.handlers.remove(&command_type);
    }

    /// Execute handlers for the given command
    fn execute_handlers(&self, command: &Command) {
        if let Some(handlers) = self.handlers.get(&command.command_type) {
            for handler in handlers {
                handler(command);
            }
        }
    }

    /// Initialize the script engine
    pub fn initialize(&mut self) -> Result<()> {
        log::info!("ScriptEngine initialized");
        Ok(())
    }

    /// Shutdown the script engine
    pub fn shutdown(&mut self) {
        self.stop();
        self.commands.clear();
        self.labels.clear();
        log::info!("ScriptEngine shutdown");
    }

    /// Load and parse a script file
    pub fn load_script<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let content = fs::read_to_string(path)
            .context("Failed to read script file")?;
        self.parse_script(&content)
    }

    /// Parse script content
    fn parse_script(&mut self, content: &str) -> Result<()> {
        self.commands.clear();
        self.labels.clear();
        self.current_index = 0;

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse commands
            if let Some(command) = self.parse_line(line) {
                if command.command_type == CommandType::Label {
                    if let Some(name) = command.params.first() {
                        self.labels.insert(name.clone(), self.commands.len());
                    }
                }
                self.commands.push(command);
            }
        }

        log::info!("Loaded script with {} commands", self.commands.len());
        Ok(())
    }

    /// Parse a single line into a command
    fn parse_line(&self, line: &str) -> Option<Command> {
        // Label
        if line.starts_with('*') {
            let name = line[1..].trim().to_string();
            return Some(Command::new(CommandType::Label, vec![name]));
        }

        // Command
        if line.starts_with('@') {
            let parts: Vec<&str> = line[1..].splitn(2, ' ').collect();
            let cmd_name = parts[0];
            let params: Vec<String> = parts
                .get(1)
                .map(|s| s.split_whitespace().map(|p| p.to_string()).collect())
                .unwrap_or_default();

            let cmd_type = match cmd_name {
                "bg" | "background" => CommandType::Background,
                "char" | "character" => CommandType::Character,
                "music" | "bgm" => CommandType::Music,
                "sound" | "sfx" => CommandType::Sound,
                "wait" => CommandType::Wait,
                "jump" => CommandType::Jump,
                "end" => CommandType::End,
                _ => CommandType::None,
            };

            return Some(Command::new(cmd_type, params));
        }

        // Dialogue (Speaker: Text)
        if let Some(pos) = line.find(':') {
            let speaker = line[..pos].trim().to_string();
            let text = line[pos + 1..].trim().to_string();
            return Some(Command::new(CommandType::Dialogue, vec![speaker, text]));
        }

        None
    }

    /// Start script execution
    pub fn start(&mut self) {
        self.running = true;
        self.paused = false;
        self.current_index = 0;
        self.execute_current();
    }

    /// Stop script execution
    pub fn stop(&mut self) {
        self.running = false;
        self.paused = false;
    }

    /// Pause execution
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume execution
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Advance to next command
    pub fn next(&mut self) {
        if self.waiting_for_input && !self.paused {
            self.waiting_for_input = false;
            self.current_index += 1;
            if self.current_index < self.commands.len() {
                self.execute_current();
            } else {
                self.running = false;
            }
        }
    }

    /// Execute the current command
    fn execute_current(&mut self) {
        if self.current_index >= self.commands.len() {
            self.running = false;
            return;
        }

        let command = &self.commands[self.current_index];
        
        // Execute registered handlers
        self.execute_handlers(command);
        
        // Handle command based on type
        match command.command_type {
            CommandType::Dialogue | CommandType::Choice => {
                self.waiting_for_input = true;
            }
            CommandType::End => {
                self.running = false;
            }
            CommandType::Label => {
                // Auto-skip labels
                self.current_index += 1;
                self.execute_current();
            }
            _ => {
                // Auto-advance for other commands
                self.current_index += 1;
                self.execute_current();
            }
        }
    }

    /// Jump to a label
    pub fn jump_to_label(&mut self, label: &str) -> bool {
        if let Some(&index) = self.labels.get(label) {
            self.current_index = index + 1;
            self.execute_current();
            true
        } else {
            false
        }
    }

    /// Get the current command
    pub fn current_command(&self) -> Option<&Command> {
        self.commands.get(self.current_index)
    }

    /// Check if waiting for input
    pub fn is_waiting_for_input(&self) -> bool {
        self.waiting_for_input
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}
