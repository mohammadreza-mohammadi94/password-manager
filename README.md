# Password Manager

A secure, terminal-based password manager built with Rust. This application provides a user-friendly TUI (Terminal User Interface) for managing passwords and credentials securely.

## Features

- Secure password storage using strong encryption
- Terminal-based user interface using TUI
- Master password protection
- Add and view credentials
- Show/hide password functionality
- Navigation using keyboard shortcuts
- Persistent storage with encryption

## Prerequisites

- Rust (1.56.0 or later)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/mohammadreza-mohammadi94/password-manager.git
cd password-manager
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

## Usage

### Key Bindings

#### Lock Screen
- `Enter` - Unlock vault with master password
- `Esc` - Exit application

#### Main Screen
- `q` - Quit application
- `a` - Add new credential
- `↑/↓` - Navigate through credentials
- `Enter` - View selected credential

#### Add Credential Screen
- `i` - Enter editing mode
- `Tab` - Switch between fields
- `Enter` - Save credential
- `Esc` - Exit editing mode/Return to main screen
- `q` - Return to main screen

#### View Credential Screen
- `s` - Show/hide password
- `q/Esc` - Return to main screen

## Security

This password manager implements several security measures:

- AES-256 encryption for stored credentials
- Argon2 key derivation for master password
- Secure memory handling
- No plaintext storage of sensitive data

## Project Structure

```
src/
├── crypto.rs      # Encryption/decryption functionality
├── main.rs        # Application entry point
├── manager.rs     # Password manager core functionality
├── models.rs      # Data structures
├── storage.rs     # File storage handling
└── ui/           
    ├── app.rs     # Application state
    ├── components.rs # UI components
    ├── handlers.rs  # Input handling
    └── mod.rs      # UI module entry
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
