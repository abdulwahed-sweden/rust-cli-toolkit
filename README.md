# Rust CLI Toolkit (RCT)

A powerful, modular command-line toolkit built with Rust featuring file operations, system utilities, task management, and an interactive shell.

[![Build Status](https://github.com/YOUR_USERNAME/rust-cli-toolkit/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/rust-cli-toolkit/actions)
[![Crates.io](https://img.shields.io/crates/v/rust-cli-toolkit.svg)](https://crates.io/crates/rust-cli-toolkit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- 📁 **File Operations**: Create, read, copy, move, delete files and directories
- 🌐 **Network Utilities**: Download files, HTTP requests, connectivity tests
- 💻 **System Information**: Process monitoring, system stats, environment management
- 📋 **Task Management**: Todo lists, project tracking, reminders
- 🔧 **Configuration Management**: Flexible settings with TOML support
- 🐚 **Interactive Shell**: Full-featured REPL with auto-completion
- 📊 **Statistics & Analytics**: Usage tracking and performance insights
- 🔄 **Macro System**: Automate complex workflows
- 🎨 **Beautiful Output**: Colored output with progress bars and icons

## 🚀 Quick Start

### Installation

#### From Crates.io
```bash
cargo install rust-cli-toolkit
```

#### From Source
```bash
git clone https://github.com/abdulwahed-sweden/rust-cli-toolkit.git
cd rust-cli-toolkit
cargo install --path .
```

### Usage

```bash
# Initialize configuration
rct init

# Start interactive shell
rct shell

# File operations
rct file create hello.txt "Hello, World!"
rct file read hello.txt

# System information
rct system info
rct system processes

# Task management
rct task add "Complete project documentation"
rct task list

# Network utilities
rct net download https://example.com/file.zip
rct net ping google.com

# Show help
rct --help
```

## 📖 Documentation

- [Installation Guide](docs/installation.md)
- [User Guide](docs/user-guide.md)
- [API Reference](docs/api.md)
- [Contributing](CONTRIBUTING.md)

## 🛠️ Development

### Prerequisites

- Rust 1.70+ 
- Git

### Building from Source

```bash
git clone https://github.com/abdulwahed-sweden/rust-cli-toolkit.git
cd rust-cli-toolkit
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
cargo clippy
```

## 📊 Project Structure

```
rust-cli-toolkit/
├── src/
│   ├── main.rs              # Application entry point
│   ├── commands/            # Command implementations
│   │   ├── mod.rs
│   │   ├── file.rs          # File operations
│   │   ├── net.rs           # Network utilities
│   │   ├── system.rs        # System information
│   │   └── task.rs          # Task management
│   ├── config/              # Configuration management
│   ├── shell/               # Interactive shell
│   ├── utils/               # Utility functions
│   └── errors/              # Error handling
├── tests/                   # Integration tests
├── docs/                    # Documentation
├── examples/                # Usage examples
└── Cargo.toml
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Add tests for new functionality
5. Run tests: `cargo test`
6. Commit your changes: `git commit -m 'Add amazing feature'`
7. Push to the branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Clap](https://github.com/clap-rs/clap) for command-line parsing
- Powered by [Tokio](https://tokio.rs/) for async runtime
- Inspired by modern CLI tools like `bat`, `exa`, and `fd`

## 📈 Roadmap

- [ ] Plugin system for custom commands
- [ ] Configuration templates
- [ ] Advanced file watching
- [ ] Cloud integration
- [ ] Performance optimizations

## 🔗 Links

- [Crates.io](https://crates.io/crates/rust-cli-toolkit)
- [Documentation](https://docs.rs/rust-cli-toolkit)
- [GitHub Issues](https://github.com/YOUR_USERNAME/rust-cli-toolkit/issues)
- [GitHub Discussions](https://github.com/YOUR_USERNAME/rust-cli-toolkit/discussions)