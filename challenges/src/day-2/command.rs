use std::env;

/// Represents the command arguments passed at the instantiation of this process.
pub struct CommandArguments {
    pub input_file_path: String
}

impl CommandArguments {
    /// Instantiates a new instance of the [CommandArguments] type based on the provided arguments configuration.
    pub fn new(input_file_path: String) -> Self {
        Self {
            input_file_path
        }
    }

    /// Instantiates a new instance of the [CommandArguments] type based on the current state of the process environment.
    pub fn from_env() -> Self {
        let mut process_arguments: Vec<String> = env::args().collect();
        let input_file_path = process_arguments.remove(1);

        Self {
            input_file_path
        }
    }
}