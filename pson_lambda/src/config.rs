#[derive(Debug)]
pub struct ArgConfig {
    pub help: bool,
    pub version: bool,
    pub interactive: bool,
    pub test: bool,
    pub file: Option<String>,
}

impl ArgConfig {
    pub fn new() -> ArgConfig {
        ArgConfig {
            help: false,
            version: false,
            interactive: false,
            test: false,
            file: None,
        }
    }
}
