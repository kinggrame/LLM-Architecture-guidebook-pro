/// Types of script commands
#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    None,
    Dialogue,
    Choice,
    Background,
    Character,
    Music,
    Sound,
    Transition,
    Wait,
    Label,
    Jump,
    End,
}

/// A script command with type and parameters
#[derive(Debug, Clone)]
pub struct Command {
    pub command_type: CommandType,
    pub params: Vec<String>,
}

impl Command {
    /// Create a new command
    pub fn new(command_type: CommandType, params: Vec<String>) -> Self {
        Self {
            command_type,
            params,
        }
    }
}
