# 🔒 Privacy-First Password Manager

A secure, TUI-based password and API key manager built with Rust. This application provides a fast, offline-first, and privacy-focused way to manage your sensitive credentials directly from your terminal.


## ✨ Features

- **Local-First Storage:** All your data is stored locally in an encrypted vault. You own your data.
- **Strong Encryption:** Utilizes **AES-256-GCM** for authenticated encryption, with a key derived from your master password using **Argon2**.
- **Password & API Key Management:** Store both traditional passwords and API keys with dedicated fields.
- **Modern TUI:** A clean, user-friendly terminal interface built with `tui-rs`.
- **Cross-Platform:** Built with Rust, it compiles and runs on Windows, macOS, and Linux.
- **Custom Theming:** Customize the application's color scheme to your liking.
- **Auto-Lock on Inactivity:** Automatically locks the vault after a configurable period of inactivity (e.g., 5 or 15 minutes).
- **Import/Export Vault:** Add functionality to export the entire vault to a standard format like CSV or JSON (with a strong warning about it being unencrypted). You could also implement an import feature to migrate from other password managers.
- **Core Functionality:**
  - Add, Edit, and Delete credentials.
  - View credential details with a show/hide toggle for secrets.
  - Reset the entire vault if needed.

## 🛠️ Built With

- **[Rust](https://www.rust-lang.org/)** - The core programming language.
- **[TUI-rs](https://github.com/fdehau/tui-rs)** - For building the terminal user interface.
- **[Crossterm](https://github.com/crossterm-rs/crossterm)** - For terminal manipulation.
- **[Ring](https://github.com/briansmith/ring)** - For cryptographic operations (AES-256-GCM).
- **[Argon2](https://github.com/bryant/argon2-rs)** - For secure key derivation.
- **[Sled](https://github.com/spacejam/sled)** - For embedded database storage.
- **[Bincode](https://github.com/bincode-org/bincode)** - For binary serialization.

## 🚀 Getting Started

### Prerequisites

- **Rust & Cargo:** Ensure you have the latest version of Rust installed. You can get it from [rustup.rs](https://rustup.rs/).

### Installation & Running

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/mohammadreza-mohammadi94/password-manager.git
    cd password-manager
    ```

2.  **Run the application:**
    ```bash
    cargo run
    ```
    The first time you run it, you will be prompted to create a new master password. This will be the only key to your vault.

## ⌨️ How to Use

### Global
- `q` or `Ctrl+C`: Quit the application.

### Lock Screen
- **Enter Password:** Type your master password and press `Enter` to unlock the vault.
- **Reset Vault:** Press `Ctrl+R` to permanently delete the current vault and start fresh. **Use with caution!**

### Main Vault Screen
- `↑`/`↓`: Navigate through the list of credentials.
- `Enter`: View the details of the selected credential.
- `a`: Switch to the "Add Credential" screen.
- `e`: Export the vault to `vault_export.json`.
- `i`: Import credentials from `vault_export.json`.
- `q`: Lock the vault and quit the application.

### View Credential Screen
- `s`: Toggle visibility of the secret (password or API key).
- `e`: Switch to "Edit" mode for the selected credential.
- `d`: Delete the selected credential.
- `q` or `Esc`: Return to the main vault screen.

### Add/Edit Credential Screen
- `i`: Enter "Insert" mode to type in a field.
- `Esc`: Exit "Insert" mode.
- `Tab`: Navigate to the next field.
- `t`: (Add mode only) Toggle between creating a `Password` or an `API Key`.
- `Enter`: Save the new or edited credential.
- `q` or `Esc`: Cancel and return to the main screen.

### Custom Theming
- `t`: Cycle through available themes.

## 🔮 Future Development

Here are some features planned for future releases:

- **Copy to Clipboard:** Securely copy secrets to the system clipboard.
- **Search/Filter:** Quickly find credentials by searching.
- **Password Generator:** A built-in tool to create strong, random passwords.

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/mohammadreza-mohammadi94/password-manager/issues).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
