use crate::ui::app::{App, View, ActiveField, InputMode};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_lock_screen_input(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Enter => {
            app.unlock_vault()?;
        }
        KeyCode::Char(c) => {
            app.master_password.push(c);
        }
        KeyCode::Backspace => {
            app.master_password.pop();
        }
        KeyCode::Esc => {
            app.should_quit = true;
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_main_screen_input(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Char('a') => {
            app.current_view = View::AddCredential;
        }
        KeyCode::Down => {
            if !app.credentials.is_empty() {
                let i = match app.selected_credential {
                    Some(i) => {
                        if i >= app.credentials.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                app.selected_credential = Some(i);
            }
        }
        KeyCode::Up => {
            if !app.credentials.is_empty() {
                let i = match app.selected_credential {
                    Some(i) => {
                        if i == 0 {
                            app.credentials.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                app.selected_credential = Some(i);
            }
        }
        KeyCode::Enter => {
            if app.selected_credential.is_some() {
                app.current_view = View::ViewCredential;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_add_credential_input(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => {
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                app.clear_form();
                app.current_view = View::Main;
            }
            KeyCode::Tab => {
                app.next_field();
            }
            KeyCode::Enter => {
                app.add_credential()?;
            }
            _ => {}
        },
        InputMode::Editing => match key.code {
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Backspace => {
                if let Some(active_field) = app.active_field {
                    match active_field {
                        ActiveField::Service => {
                            app.service_input.pop();
                        }
                        ActiveField::Username => {
                            app.username_input.pop();
                        }
                        ActiveField::Password => {
                            app.password_input.pop();
                        }
                        ActiveField::Notes => {
                            app.notes_input.pop();
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                if let Some(active_field) = app.active_field {
                    match active_field {
                        ActiveField::Service => {
                            app.service_input.push(c);
                        }
                        ActiveField::Username => {
                            app.username_input.push(c);
                        }
                        ActiveField::Password => {
                            app.password_input.push(c);
                        }
                        ActiveField::Notes => {
                            app.notes_input.push(c);
                        }
                    }
                }
            }
            _ => {}
        },
    }
    Ok(())
}

pub fn handle_view_credential_input(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.current_view = View::Main;
            app.show_password = false; // Reset when leaving view
        }
        KeyCode::Char('s') => {
            app.show_password = !app.show_password;
        }
        _ => {}
    }
    Ok(())
}