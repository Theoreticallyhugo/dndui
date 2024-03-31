use crate::app::{App, AppResult, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.get_input_mode() {
        InputMode::Normal => {
            match key_event.code {
                // Exit application on `q`
                KeyCode::Char('q') => {
                    app.quit();
                }
                // // Exit application on `ESC` or `q`
                // KeyCode::Esc | KeyCode::Char('q') => {
                //     app.quit();
                // }
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                KeyCode::Char('f') => {
                    app.character.load_character();
                }
                KeyCode::Esc => {
                    app.stop_input();
                }
                KeyCode::Char('d') => {
                    app.start_healing();
                }
                KeyCode::Char('t') => {
                    app.start_temp_healing();
                }
                KeyCode::Char('i') | KeyCode::Char('I') => {
                    app.start_inspiration();
                }
                KeyCode::Char('?') => {
                    app.enter_help_screen();
                }
                // Counter handlers
                KeyCode::Right | KeyCode::Char('l') => {
                    app.increment_counter();
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    app.decrement_counter();
                }
                // Other handlers you could add here.
                _ => {}
            }
        },
        InputMode::HelpScreen => {
            match key_event.code {
                // Exit application on `q`
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.leave_help_screen();
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                // Counter handlers
                KeyCode::Right | KeyCode::Char('l') => {
                    app.increment_counter();
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    app.decrement_counter();
                }
                _ => {}
            }
        },
        InputMode::Healing | InputMode::TempHealing => {
            match key_event.code {
                KeyCode::Esc => {
                    app.clear_input();
                    app.stop_input();
                },
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                KeyCode::Enter => app.submit_message(),
                KeyCode::Char('-') => {
                    if app.get_input_length() == 0 {
                        app.enter_char('-');
                    }
                }
                KeyCode::Char('+') => {
                    if app.get_input_length() == 0 {
                        app.enter_char('+');
                    }
                }
                KeyCode::Char(to_insert) => {
                    if to_insert.is_ascii_digit() && app.get_input_length() < 6 {
                        app.enter_char(to_insert);
                    }
                }
                KeyCode::Backspace => {
                    app.delete_char();
                }
                KeyCode::Left => {
                    app.move_cursor_left();
                }
                KeyCode::Right => {
                    app.move_cursor_right();
                }
                _ => {}
            }
        },
        InputMode::Inspiration => {
            match key_event.code {
                KeyCode::Esc => {
                    app.clear_input();
                    app.stop_input();
                },
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                },
                KeyCode::Char('i') => {
                    app.character.set_inspiration(false);
                    app.stop_input();
                },
                KeyCode::Char('I') => {
                    app.character.set_inspiration(true);
                    app.stop_input();
                },
                _ => {}
            }
        },
    }
    Ok(())
}

/// Handles the mouse events and updates the state of [`App`].
pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match mouse_event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            app.increment_counter();
        }
        MouseEventKind::Down(MouseButton::Right) => {
            app.decrement_counter();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
