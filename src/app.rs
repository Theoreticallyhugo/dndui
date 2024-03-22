use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub enum InputMode {
    Normal,
    Damaging,
}

/// Application.
// #[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    /// counter
    counter: u8,
    /// current input mode
    input_mode: InputMode,
    /// current value of the input box
    input: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            input_mode: InputMode::Normal,
            input: String::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn get_counter(&self) -> u8 {
        self.counter
    }

    pub fn get_running(&self) -> bool {
        self.running
    }

    pub fn get_input_mode(&self) -> &InputMode {
        &self.input_mode
    }

    pub fn stop_input(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn start_damaging(&mut self) {
        self.input_mode = InputMode::Damaging;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
