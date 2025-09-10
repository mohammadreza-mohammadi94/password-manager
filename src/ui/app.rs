use crate::manager::PasswordManager;
use crate::models::{Credential, EntryType};

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
    pub error_message: Option<String>,
    pub show_secret: bool,
    pub active_field: Option<ActiveField>,
    pub is_active_input: bool,
    pub entry_type: EntryType,
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
            error_message: None,
            show_secret: false,
            active_field: Some(ActiveField::Service),
            is_active_input: true,
            entry_type: EntryType::Password,
        })
    }

    pub fn unlock_vault(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.password_manager.unlock(&self.master_password)? {
            self.current_view = View::Main;
            self.load_credentials()?;
            self.error_message = None;
        } else {
            self.error_message = Some("Invalid password".to_string());
            self.master_password.clear();
        }
        Ok(())
    }

    pub fn load_credentials(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.credentials = self.password_manager.get_credentials()?;
        Ok(())
    }

    pub fn add_credential(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Store values locally to avoid multiple borrows
        let entry_type = self.entry_type.clone();
        let service = self.service_input.clone();
        let username = self.username_input.clone();
        let secret = self.secret_input.clone();
        let notes = self.notes_input.clone();
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
                )?;
            },
            EntryType::ApiKey => {
                self.password_manager.add_api_key(
                    service,
                    username,
                    secret,
                    notes,
                    is_active,
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
            Some(ActiveField::Notes) => {
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

    pub fn load_selected_credential_for_edit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(idx) = self.selected_credential {
            let credential = &self.credentials[idx];
            self.service_input = credential.service.clone();
            self.username_input = credential.username.clone();
            self.secret_input = String::from_utf8(credential.secret.clone()).unwrap_or_default();
            self.notes_input = credential.notes.clone();
            self.is_active_input = credential.is_active;
            self.entry_type = credential.entry_type.clone();
            self.selected_id = Some(credential.id.clone());
            self.input_mode = InputMode::Normal;  // Start in normal mode to allow 'i' to enter edit mode
            self.active_field = Some(ActiveField::Service);
            self.show_secret = false;  // Reset show secret when entering edit mode
        }
        Ok(())
    }
}