# ttray - Terminal System Tray Viewer

`ttray` is a terminal-based application that allows you to view and interact with system tray applications directly from your terminal. It provides a simple and efficient interface to manage system tray items without leaving your terminal environment.

> This project was inspired by and borrows code from [tray-tui](https://github.com/Levizor/tray-tui).

It aims to provide a cleaner ui, and better usability.

## Features

- View all system tray applications in a list interface
- Browse and interact with system tray menus
- Keyboard-driven navigation for efficiency
- Terminal-based UI using [ratatui](https://github.com/ratatui-org/ratatui)

## Installation

### From Source

```sh
# Clone the repository
git clone https://github.com/aurreland/ttray.git
cd ttray

# Build and install the application
cargo install --path .
```

## Usage

```sh
# Run ttray
ttray
```

### Command-line Options

- `--debug`, `-d`: Enable debug logging to `app.log` file
- `--help`, `-h`: Display help information
- `--version`, `-V`: Display version information

### Keyboard Controls

| Key        | Action                      |
| ---------- | --------------------------- |
| `q`, `Esc` | Quit the application        |
| `Ctrl+C`   | Quit the application        |
| `←`, `h`   | Select previous app         |
| `→`, `l`   | Select next app             |
| `↑`, `k`   | Move up in action menu      |
| `↓`, `j`   | Move down in action menu    |
| `Space`    | Toggle expand/collapse menu |
| `Enter`    | Activate selected action    |

## ToDO

- [ ] Add configuration file support
- [ ] Add keybindings support
- [ ] Add themes and customization options

## License

[GPLv3](LICENSE)

## Acknowledgments

- [tray-tui](https://github.com/Levizor/tray-tui) - Original inspiration for this project and source of some borrowed code
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library used for the interface
- [tui-tree-widget](https://github.com/EdJoPaTo/tui-rs-tree-widget) - Tree widget used for displaying menus
- [system_tray](https://github.com/FedeDP/system-tray-rs) - Rust library for interacting with system tray applications

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
