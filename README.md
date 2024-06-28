# Stash

<p align="center">
  <a href="https://github.com/aymanfarsi/Stash"><img src="assets/stash.png" alt="Stash" height="120" /></a>
</p>

<p align="center">
  <strong>Stash is a new way to manage your bookmarks for no need to keep tabs open.</strong>
</p>

## Contents
- [Features](#features)
- [Installation](#installation)
- [Building from source](#building-from-source)
- [Contributing](#contributing)
- [Credits](#credits)
- [License](#license)

## Features

- **Cross-platform**: Stash is built using Rust and egui, making it highly portable and compatible with Windows, macOS, and Linux.
- **Secure**: Stash only stores your bookmarks locally on your device, ensuring your privacy and security.
- Portable: Stash is a single binary that you can run from anywhere on your system. Also, all configuration files are stored in your documents folder.

## Installation

To use Stash, you only need to have Rust installed on your system if you want to build it from source. Otherwise, you can download the pre-built binaries for your platform from the [releases page](https://www.github.com/aymanfarsi/Stash/releases).

## Building from source

To install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. Clone the repository:

```bash
git clone https://github.com/aymanfarsi/Stash.git
```

2. Change into the project directory and rename it to avoid issues with building the executable:

```bash
mv Stash stash
cd stash
```

3. Build the project:

```bash
cargo build --release
```

4. The binary will be located in the `target/release` directory. You can run it using:

```bash
./target/release/stash
```

5. You can also install the binary to your system using:

```bash
cargo install --path .
```

6. Otherwise, run the install script:

```bash
./install-stash # Unix systems
sh install-stash # Windows
```

7. You can now run the binary using `stash` or search for it in your system's application menu.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request if you have any ideas, bug reports, or feature requests.

1. Fork the repository and clone it to your local machine.
2. Create a new branch for your changes.
3. Make your changes and commit them.
4. Push the changes.
5. Submit a pull request.

## Credits

Stash is built using the following technologies:

- [Rust](https://www.rust-lang.org/): A systems programming language that focuses on safety, speed, and concurrency.
- [egui](https://www.github.com/emilk/egui): A simple, fast, and highly portable immediate mode GUI library.

## License

Stash is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
