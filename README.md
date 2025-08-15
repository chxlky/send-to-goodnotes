# Send to GoodNotes

A cross-platform desktop application for easily sending PDF files to GoodNotes via email. Built with Rust and Iced for a native, fast, and secure experience.

## Features

- 📁 **File Selection**: Choose PDF files using a file dialog or drag-and-drop interface
- ✏️ **File Management**: Rename files before sending with inline editing
- 📧 **Email Integration**: Send PDFs directly to your GoodNotes email address
- 🔒 **Secure Settings**: Encrypted storage of email credentials using AES-256-GCM
- 🎨 **Modern UI**: Clean, dark-themed interface built with Iced
- 🖱️ **Intuitive UX**: Click outside to commit edits, visual feedback for all actions
- ⚡ **Performance**: Native Rust performance with GPU-accelerated rendering

## How It Works

1. **Add Files**: Use the "Open file(s)" button or drag and drop PDF files into the application
2. **Edit Names**: Click on any file name to rename it (useful for organizing in GoodNotes)
3. **Configure Email**: Set up your SMTP settings in the secure settings panel
4. **Send**: Click "Send" to email your PDFs directly to your GoodNotes import address

## Technologies Used

### Core Framework

- **[Rust](https://www.rust-lang.org/)** - Systems programming language for performance and safety
- **[Iced](https://iced.rs/)** - Cross-platform GUI framework with GPU acceleration

### Email & Networking

- **[lettre](https://github.com/lettre/lettre)** - SMTP email client for sending attachments
- **SMTP** - Standard email protocol for reliable delivery

### Security & Storage

- **[aes-gcm](https://github.com/RustCrypto/AEADs)** - AES-256-GCM encryption for settings storage
- **[base64](https://github.com/marshallpierce/rust-base64)** - Encoding for encrypted data
- **[dirs](https://codeberg.org/dirs/dirs-rs)** - Platform-appropriate config directories
- **[whoami](https://github.com/libcala/whoami)** - Machine identification for encryption keys

### File Handling & UI

- **[rfd](https://github.com/PolyMeilex/rfd)** - Native file dialogs
- **[serde](https://serde.rs/)** - Serialization for configuration management
- **GPU Rendering** - Hardware-accelerated UI via wgpu backend

### Development Environment

- **[Nix](https://nixos.org/)** - Reproducible development environment
- **Cargo** - Rust package manager and build system

## Installation

### Prerequisites

- Rust 1.88+ (or use the provided Nix flake)
- Git

### Building from Source

```bash
# Clone the repository
git clone git@github.com:chxlky/send-to-goodnotes.git
cd send-to-goodnotes

# Build and run
cargo run --release
```

### Using Nix (Recommended)

```bash
# Enter development environment
nix develop

# Build and run
cargo run --release

# Optionally, compile for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## Configuration

### Email Settings

The application requires SMTP configuration to send emails:

1. **SMTP Host**: Your email provider's SMTP server (e.g., `smtp.gmail.com`)
2. **SMTP Port**: Usually 587 for TLS or 465 for SSL
3. **From Email**: Your email address
4. **To Email**: Your GoodNotes import email address
5. **App Password**: App-specific password (recommended over regular passwords)

### Setting Up Gmail

1. Enable 2-factor authentication
2. Generate an app-specific password from the Google Cloud Console
3. Use `smtp.gmail.com` with port `587`

### Security

- All email credentials are encrypted using AES-256-GCM and stored locally
- Encryption keys are derived from machine-specific identifiers
- Settings are stored in platform-appropriate configuration directories

## Usage Tips

- **File Limits**: Maximum 5 PDFs per email
- **Drag & Drop**: Works on most platforms (may have limitations on some Linux desktop environments)
- **Editing**: Click any filename to rename it, press Enter or click outside to save
- **Status Feedback**: Color-coded status messages show success (green) or errors (red)

## Project Structure

```text
src/
├── main.rs                   # Main application logic and message handling
├── email.rs                  # Email sending functionality
├── config.rs                 # Encrypted settings management
└── app/
    ├── mod.rs                # Module declarations
    ├── messages.rs           # Application message types
    ├── state.rs              # Application state management
    ├── view.rs               # UI rendering and styling
    └── widgets/
        ├── mod.rs            # Widget module declarations
        └── outside_commit.rs # Custom widget for edit behavior
```

## License

This project is licensed under the GNU General Public License v3.0. See the LICENSE file for details.

## Troubleshooting

### Email Not Sending

- Verify SMTP settings are correct
- Check that app passwords are enabled for your email provider
- Ensure firewall isn't blocking SMTP ports

### Drag & Drop Not Working

- Some Linux desktop environments have limitations with drag & drop
- Use the "Open file(s)" button as an alternative

### File Not Found Errors

- Ensure PDF files haven't been moved or deleted after selection
- Try re-adding the files if they've been relocated
