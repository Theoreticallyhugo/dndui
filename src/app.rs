use std::error;
use crate::character::Character;
use crate::character_limbs::advantage::Advantage;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Normal,
    Healing,
    TempHealing,
    Inspiration,
    HelpScreen,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    /// counter
    counter: u8,
    /// current input mode
    input_mode: InputMode,
    /// current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    cursor_position: usize,
    /// database for charater
    pub character: Character,

    help_screen_shown: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            input_mode: InputMode::Normal,
            input: String::new(),
            cursor_position: 0,
            character: Character::new(),
            help_screen_shown: false,
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

    pub fn get_input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn get_input(&self) -> &String {
        &self.input
    }

    pub fn get_input_length(&self) -> usize {
        self.input.len()
    }

    pub fn get_help_screen_shown(&self) -> bool {
        self.help_screen_shown
    }

    pub fn enter_help_screen(&mut self) {
        self.input_mode = InputMode::HelpScreen;
        self.help_screen_shown = true;
    }

    pub fn leave_help_screen(&mut self) {
        self.input_mode = InputMode::Normal;
        self.help_screen_shown = false;
    }

    pub fn stop_input(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn start_healing(&mut self) {
        self.input_mode = InputMode::Healing;
        self.enter_char('+');
    }

    pub fn start_temp_healing(&mut self) {
        self.input_mode = InputMode::TempHealing;
        self.enter_char('+');
    }

    pub fn start_inspiration(&mut self) {
        self.input_mode = InputMode::Inspiration;
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    pub fn input_to_neg_sign(&mut self) {
        if self.get_input_length() == 0 {
            self.enter_char('-');
        } else {
            self.input.remove(0);
            self.input.insert(0, '-');
        }
    }

    pub fn input_to_pos_sign(&mut self) {
        if self.get_input_length() == 0 {
            self.enter_char('+');
        } else {
            self.input.remove(0);
            self.input.insert(0, '+');
        }
    }

    pub fn input_get_sign(&self) -> char {
        self.input.chars().nth(0).unwrap()
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.reset_cursor();
    }

    pub fn submit_message(&mut self) {
        // self.messages.push(self.input.clone());
        match self.get_input_mode() {
            InputMode::Healing => {
                self.character.add_healing(self.get_input().parse::<i32>().unwrap_or(0));
            },
            InputMode::TempHealing => {
                self.character.add_temp_healing(self.get_input().parse::<i32>().unwrap_or(0));
            },
            _ => {},
        }
        self.clear_input();
        self.stop_input();
    }

    pub fn get_dot(&self, infill: bool) -> &str {
        if infill {
            "●" 
        } else {
            "○" 
        }
    }

    pub fn get_advantage_letter(&self, advantage: Advantage) -> &str {
        match advantage {
            Advantage::No => " ",
            Advantage::Advantage => "A",
            Advantage::Disadvantage => "D",
        }
    }

    pub fn increment_counter(&mut self) {
        if self.counter == 6 {
            self.counter = 0;
        } else if let Some(res) = self.counter.checked_add(1) { 
            self.counter = res; 
        }
    }

    pub fn decrement_counter(&mut self) {
        if self.counter == 0 {
            self.counter = 6;
        } else if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
