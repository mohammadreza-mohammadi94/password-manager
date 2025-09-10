use crate::manager::PasswordManager;
use crate::models::{Credential, EntryType};
use zxcvbn::zxcvbn;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, PartialEq, Clone)]
pub enum View {
    LockScreen,
    Main,
    AddCredential,
    ViewCredential,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActiveField {
    Service,
    Username,
    Secret,
    Notes,
    Tags,
    IsActive,
}

pub struct App {
    pub password_manager: PasswordManager,
    #[allow(dead_code)]
    pub input_mode: InputMode,
    pub master_password: String,
    pub current_view: View,
    pub credentials: Vec<Credential>,
    pub should_quit: bool,
    pub selected_credential: Option<usize>,
    pub selected_id: Option<String>,
    pub service_input: String,
    pub username_input: String,
    pub secret_input: String,
    pub notes_input: String,
    pub tags_input: String,
    pub search_query: String,
    pub password_strength: Option<u8>,
    pub error_message: Option<String>,
    pub info_message: Option<String>,
    pub show_secret: bool,
    pub active_field: Option<ActiveField>,
    pub is_active_input: bool,
    pub entry_type: EntryType,
    pub last_activity: Instant,
    pub inactivity_duration: Duration,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            password_manager: PasswordManager::new()?,
            input_mode: InputMode::Normal,
            master_password: String::new(),
            current_view: View::LockScreen,
            credentials: Vec::new(),
            should_quit: false,
            selected_credential: None,
            selected_id: None,
            service_input: String::new(),
            username_input: String::new(),
            secret_input: String::new(),
            notes_input: String::new(),
            tags_input: String::new(),
            search_query: String::new(),
            password_strength: None,
            error_message: None,
            info_message: None,
            show_secret: false,
            active_field: Some(ActiveField::Service),
            is_active_input: true,
            entry_type: EntryType::Password,
            last_activity: Instant::now(),
            inactivity_duration: Duration::from_secs(5 * 60), // 5 minutes
        })
    }

    pub fn unlock_vault(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.master_password.is_empty() {
            self.error_message = Some("Password cannot be empty".to_string());
            return Ok(());
        }
        match self.password_manager.unlock(&self.master_password) {
            Ok(true) => {
                self.current_view = View::Main;
                self.load_credentials()?;
                self.error_message = None;
                self.reset_activity_timer();
            }
            Ok(false) => {
                self.error_message = Some("Invalid password".to_string());
                self.master_password.clear();
            }
            Err(e) => {
                self.error_message = Some(format!("Error: {}", e));
                self.master_password.clear();
            }
        }
        Ok(())
    }

    pub fn lock_vault(&mut self) {
        self.password_manager.lock();
        self.master_password.clear();
        self.current_view = View::LockScreen;
        self.credentials.clear();
        self.selected_credential = None;
        self.error_message = None;
        self.clear_form();
    }

    pub fn check_inactivity(&mut self) {
        if self.current_view != View::LockScreen && self.last_activity.elapsed() > self.inactivity_duration {
            self.lock_vault();
        }
    }

    pub fn reset_activity_timer(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn load_credentials(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.credentials = self.password_manager.get_credentials()?;
        self.filter_credentials();
        Ok(())
    }

    pub fn filter_credentials(&mut self) {
        let query = self.search_query.to_lowercase();
        if query.is_empty() {
            self.credentials = self.password_manager.get_credentials().unwrap_or_default();
        } else {
            self.credentials = self.password_manager.get_credentials().unwrap_or_default()
                .into_iter()
                .filter(|c| {
                    c.service.to_lowercase().contains(&query) ||
                    c.username.to_lowercase().contains(&query) ||
                    c.tags.iter().any(|t| t.to_lowercase().contains(&query))
                })
                .collect();
        }
    }

    pub fn add_credential(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Store values locally to avoid multiple borrows
        let entry_type = self.entry_type.clone();
        let service = self.service_input.clone();
        let username = self.username_input.clone();
        let secret = self.secret_input.clone();
        let notes = self.notes_input.clone();
        let tags = self.tags_input.split(',').map(|s| s.trim().to_string()).collect();
        let is_active = self.is_active_input;

        // Clear form state immediately
        self.clear_form();

        // Perform the add operation
        match entry_type {
            EntryType::Password => {
                self.password_manager.add_password(
                    service,
                    username,
                    secret,
                    notes,
                    tags,
                )?;
            },
            EntryType::ApiKey => {
                self.password_manager.add_api_key(
                    service,
                    username,
                    secret,
                    notes,
                    is_active,
                    tags,
                )?;
            }
        }

        // Reload credentials
        self.load_credentials()?;

        Ok(())
    }

    pub fn remove_selected_credential(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(id) = &self.selected_id {
            self.password_manager.remove_credential(id)?;
            self.load_credentials()?;
            self.selected_credential = None;
            self.selected_id = None;
        }
        Ok(())
    }

    pub fn update_selected_credential(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Store values locally to avoid multiple borrows
        let id = self.selected_id.as_ref().ok_or("No credential selected")?.clone();
        let service = self.service_input.clone();
        let username = self.username_input.clone();
        let secret = self.secret_input.clone();
        let notes = self.notes_input.clone();
        let tags = self.tags_input.split(',').map(|s| s.trim().to_string()).collect();
        let is_active = self.is_active_input;

        // Clear form state immediately
        self.clear_form();
        self.selected_credential = None;
        self.selected_id = None;

        // Perform the update
        self.password_manager.update_credential(
            &id,
            Some(service),
            Some(username),
            Some(secret),
            Some(notes),
            Some(is_active),
            Some(tags),
        )?;

        // Reload credentials
        self.load_credentials()?;

        Ok(())
    }


    pub fn clear_form(&mut self) {
        self.service_input.clear();
        self.username_input.clear();
        self.secret_input.clear();
        self.notes_input.clear();
        self.tags_input.clear();
        self.password_strength = None;
        self.is_active_input = true;
        self.active_field = Some(ActiveField::Service);
        self.selected_id = None;  // Clear selected ID when clearing form
        self.entry_type = EntryType::Password;  // Reset to password type
        self.input_mode = InputMode::Normal;  // Reset to normal mode
        self.show_secret = false;  // Reset show secret
    }

    pub fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.password_manager.reset()?;
        self.master_password.clear();
        self.current_view = View::LockScreen;
        self.credentials.clear();
        self.selected_credential = None;
        self.error_message = None;
        self.clear_form();
        Ok(())
    }

    pub fn next_field(&mut self) {
        self.active_field = match self.active_field {
            Some(ActiveField::Service) => Some(ActiveField::Username),
            Some(ActiveField::Username) => Some(ActiveField::Secret),
            Some(ActiveField::Secret) => Some(ActiveField::Notes),
            Some(ActiveField::Notes) => Some(ActiveField::Tags),
            Some(ActiveField::Tags) => {
                if self.entry_type == EntryType::ApiKey {
                    Some(ActiveField::IsActive)
                } else {
                    Some(ActiveField::Service)
                }
            },
            Some(ActiveField::IsActive) => Some(ActiveField::Service),
            None => Some(ActiveField::Service),
        };
    }

    pub fn update_password_strength(&mut self) {
        if self.secret_input.is_empty() {
            self.password_strength = None;
        } else {
            let entropy = zxcvbn(&self.secret_input, &[]).unwrap();
            self.password_strength = Some(entropy.score());
        }
    }

    pub fn load_selected_credential_for_edit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(idx) = self.selected_credential {
            let credential = &self.credentials[idx];
            self.service_input = credential.service.clone();
            self.username_input = credential.username.clone();
            self.secret_input = String::from_utf8(credential.secret.clone()).unwrap_or_default();
            self.notes_input = credential.notes.clone();
            self.tags_input = credential.tags.join(", ");
            self.is_active_input = credential.is_active;
            self.entry_type = credential.entry_type.clone();
            self.selected_id = Some(credential.id.clone());
            self.input_mode = InputMode::Normal;  // Start in normal mode to allow 'i' to enter edit mode
            self.active_field = Some(ActiveField::Service);
            self.show_secret = false;  // Reset show secret when entering edit mode
            self.update_password_strength();
        }
        Ok(())
    }
}
