use crate::ui::app::{App, View, ActiveField, InputMode};
use crate::models::EntryType;
use crossterm::event::{self, KeyCode, KeyEvent};

pub fn handle_lock_screen_input(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    if key.modifiers.contains(event::KeyModifiers::CONTROL) && key.code == KeyCode::Char('r') {
        app.reset()?;
        app.error_message = Some("Password vault has been reset. Create a new master password.".to_string());
    } else {
        match key.code {
            KeyCode::Enter => {
                app.unlock_vault()?;
            }
            KeyCode::Char(c) => {
                app.master_password.push(c);
                app.error_message = None; 
            }
            KeyCode::Backspace => {
                app.master_password.pop();
                app.error_message = None; 
            }
            KeyCode::Esc => {
                app.should_quit = true;
            }
            _ => {}
        }
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
            if let Some(selected_index) = app.selected_credential {
                if let Some(credential) = app.credentials.get(selected_index) {
                    app.selected_id = Some(credential.id.clone());
                    app.current_view = View::ViewCredential;
                }
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
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Char('t') => {
                if app.selected_id.is_none() {
                    app.entry_type = match app.entry_type {
                        EntryType::Password => EntryType::ApiKey,
                        EntryType::ApiKey => EntryType::Password,
                    };
                    app.active_field = Some(ActiveField::Service);
                }
            }
            KeyCode::Tab => {
                app.next_field();
            }
            KeyCode::Enter => {
                // First save to a temporary variable to avoid state issues
                let is_update = app.selected_id.is_some();
                
                // Reset states before performing operation
                app.input_mode = InputMode::Normal;
                app.current_view = View::Main;

                if let Err(e) = if is_update {
                    app.update_selected_credential()
                } else {
                    app.add_credential()
                } {
                    app.error_message = Some(format!("Error: {}", e));
                    app.current_view = View::AddCredential;
                }
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
                        ActiveField::Secret => {
                            app.secret_input.pop();
                        }
                        ActiveField::Notes => {
                            app.notes_input.pop();
                        }
                        ActiveField::IsActive => {
                            app.is_active_input = !app.is_active_input;
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
                        ActiveField::Secret => {
                            app.secret_input.push(c);
                        }
                        ActiveField::Notes => {
                            app.notes_input.push(c);
                        }
                        ActiveField::IsActive => {
                            // IsActive is toggled with backspace or space
                            if c == ' ' {
                                app.is_active_input = !app.is_active_input;
                            }
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
            app.show_secret = false; // Reset when leaving view
        }
        KeyCode::Char('s') => {
            app.show_secret = !app.show_secret;
        }
        KeyCode::Char('e') => {
            // If a credential is selected, load it for editing
            if app.selected_credential.is_some() {
                app.load_selected_credential_for_edit()?;
                app.current_view = View::AddCredential;
            }
        }
        KeyCode::Char('d') => {
            app.remove_selected_credential()?;
            app.current_view = View::Main;
        }
        _ => {}
    }
    Ok(())
}