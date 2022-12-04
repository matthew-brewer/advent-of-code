use std::env;

/// A structure responsible for organizing the arguments passed at process instantiation to influence the behavior of the program.
pub struct Arguments {
    /// The filesystem path of the input file that should be evaluated.
    pub input_file_path: String
}

impl Arguments {
    /// Constructs a new [Arguments] instance based on the environment ([std::env]) within which this process is running.
    pub fn from_env() -> Self {
        // Retrieve the arguments passed at process instantiation
        let args: Vec<String> = env::args().collect();

        // Extract the input file path from the collection of arguments
        let input_file_path = args.get(2).expect("Missing input file path");

        // Build the new `Arguments` instance
        Arguments {
            input_file_path: input_file_path.to_owned()
        }
    }
}
