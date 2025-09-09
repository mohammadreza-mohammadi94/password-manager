use crate::manager::PasswordManager;
use crate::models::Credential;

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
    Password,
    Notes,
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
    pub service_input: String,
    pub username_input: String,
    pub password_input: String,
    pub notes_input: String,
    pub error_message: Option<String>,
    pub show_password: bool,
    pub active_field: Option<ActiveField>,
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
            service_input: String::new(),
            username_input: String::new(),
            password_input: String::new(),
            notes_input: String::new(),
            error_message: None,
            show_password: false,
            active_field: Some(ActiveField::Service),
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
        self.password_manager.add_credential(
            self.service_input.clone(),
            self.username_input.clone(),
            self.password_input.clone(),
            self.notes_input.clone(),
        )?;
        self.password_manager.save()?;
        self.load_credentials()?;
        self.clear_form();
        self.current_view = View::Main;
        Ok(())
    }

    pub fn clear_form(&mut self) {
        self.service_input.clear();
        self.username_input.clear();
        self.password_input.clear();
        self.notes_input.clear();
        self.active_field = Some(ActiveField::Service);
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
            Some(ActiveField::Username) => Some(ActiveField::Password),
            Some(ActiveField::Password) => Some(ActiveField::Notes),
            Some(ActiveField::Notes) => Some(ActiveField::Service),
            None => Some(ActiveField::Service),
        };
    }
}