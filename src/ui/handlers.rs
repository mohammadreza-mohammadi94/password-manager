use crate::ui::app::{App, View};
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
            std::process::exit(0);
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_main_screen_input(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('q') => {
            std::process::exit(0);
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
    match key.code {
        KeyCode::Enter => {
            app.add_credential()?;
        }
        KeyCode::Esc => {
            app.clear_form();
            app.current_view = View::Main;
        }
        KeyCode::Tab => {
            // In a real implementation, we would cycle through fields
        }
        KeyCode::Backspace => {
            // Handle backspace for active field
            // Simplified for brevity
        }
        KeyCode::Char(c) => {
            // Handle character input for active field
            // Simplified for brevity
            app.service_input.push(c);
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_view_credential_input(app: &mut App, _key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    app.current_view = View::Main;
    Ok(())
}