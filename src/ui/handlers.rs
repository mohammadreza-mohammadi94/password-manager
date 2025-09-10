use crate::ui::app::{App, View, ActiveField, InputMode};
use crate::models::EntryType;
use crossterm::event::{self, KeyCode, KeyEvent};
use clipboard::{ClipboardProvider, ClipboardContext};
use std::thread;
use std::time::Duration;

pub fn handle_lock_screen_input(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    if key.modifiers.contains(event::KeyModifiers::CONTROL) && key.code == KeyCode::Char('r') {
        app.reset()?;
        app.error_message = Some("Password vault has been reset. Create a new master password.".to_string());
    } else {
        match key.code {
            KeyCode::Enter => {
                if !app.master_password.is_empty() {
                    app.unlock_vault()?;
                } else {
                    app.error_message = Some("Password cannot be empty".to_string());
                }
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
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('q') => {
                app.should_quit = true;
            }
            KeyCode::Char('e') => {
                app.export_vault()?;
            }
            KeyCode::Char('i') => {
                app.import_vault()?;
            }
            KeyCode::Char('a') => {
                app.current_view = View::AddCredential;
            }
            KeyCode::Char('/') => {
                app.input_mode = InputMode::Editing;
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
        },
        InputMode::Editing => match key.code {
            KeyCode::Char(c) => {
                app.search_query.push(c);
                app.filter_credentials();
            }
            KeyCode::Backspace => {
                app.search_query.pop();
                app.filter_credentials();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
                app.search_query.clear();
                app.filter_credentials();
            }
            KeyCode::Enter => {
                app.input_mode = InputMode::Normal;
            }
            _ => {}
        },
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
                            app.update_password_strength();
                        }
                        ActiveField::Notes => {
                            app.notes_input.pop();
                        }
                        ActiveField::Tags => {
                            app.tags_input.pop();
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
                            app.update_password_strength();
                        }
                        ActiveField::Notes => {
                            app.notes_input.push(c);
                        }
                        ActiveField::Tags => {
                            app.tags_input.push(c);
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
        KeyCode::Char('c') => {
            if let Some(index) = app.selected_credential {
                if let Some(cred) = app.credentials.get(index) {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    let secret = String::from_utf8_lossy(&cred.secret).to_string();
                    ctx.set_contents(secret).unwrap();
                    app.info_message = Some("Copied to clipboard!".to_string());

                    thread::spawn(|| {
                        thread::sleep(Duration::from_secs(30));
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents("".to_string()).unwrap();
                    });
                }
            }
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
