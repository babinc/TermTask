# TermTask 

A beautiful, keyboard-driven terminal todo application built with Rust. Features a split-screen interface, markdown support, multiple color themes, and JSON storage for easy version control.

![TermTask Demo](demo.gif)

## Features

- ✨ **Split-screen interface** - Active todos on the left, completed on the right
- 🎨 **Multiple color themes** - Catppuccin Mocha, Nord, Dracula, and High Contrast
- 📝 **Markdown support** - Use markdown formatting in todo descriptions
- ⌨️ **Keyboard-driven** - Fast and efficient workflow without touching the mouse
- 💾 **JSON storage** - Human-readable, git-friendly todo storage
- 🔧 **Configurable** - Customizable themes and adjustable split-screen ratio
- 🌍 **Full Unicode support** - Use emojis and international characters in your todos

## Installation

### From Source

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed, then:

```bash
# Clone the repository
git clone https://github.com/babinc/TermTask.git
cd TermTask

# Build and install
cargo install --path .
```

### Using Cargo

```bash
cargo install termtask
```

## Usage

### Running TermTask

```bash
# Launch with default todo file (~/.local/share/termtask/todos.json)
termtask

# Use a custom todo file
termtask --file my_todos.json

# Force global todo storage
termtask --global

# Initialize project-specific todos in current directory
termtask init

# Initialize personal project todos (.todo.json)
termtask init --personal
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `+` | Add new todo |
| `r` | Edit selected todo (title and description) |
| `Space` | Toggle todo completion status |
| `e` | Expand/collapse todo description |
| `d` | Delete selected todo |
| `Tab` | Switch between active and completed panes |
| `↑/↓` | Navigate through todos |
| `+/=` | Increase split ratio (more space for active todos) |
| `-` | Decrease split ratio (more space for completed todos) |
| `t` | Quick theme toggle |
| `s` | Open settings modal |
| `q` | Quit application |

### Vim Mode in Text Editor

TermTask includes Vim keybindings in the text editor for adding and editing todos:

- `:w` or `:x` - Save and close
- `:q` - Cancel without saving
- `Esc` - Exit insert mode
- Standard vim motions and editing commands within the text editor

Vim mode can be enabled in the configuration file.

### Markdown Support

Todo descriptions support markdown formatting including headers, lists, code blocks, tables, and blockquotes. The markdown is stored in the todo descriptions and displayed when expanded.

### Configuration

TermTask follows XDG Base Directory specifications:

- **Todos**: `~/.local/share/termtask/todos.json`
- **Config**: `~/.config/termtask/config.toml`

The configuration file allows you to customize the appearance and behavior:

```toml
theme = "CatppuccinMocha"  # Options: CatppuccinMocha, Nord, Dracula, TokyoNight, OneDark, GruvboxDark, Monokai, SolarizedDark

[ui]
split_ratio = 50  # Percentage for active todos pane (0-100)
date_format = "Relative"  # Options: Relative, Absolute, None
vim_mode = false
show_completed_count = true
auto_expand_descriptions = false
compact_mode = false
status_bar_visible = true
```

### Themes

Choose from 8 beautiful built-in themes:
- **CatppuccinMocha** - Soft pastel colors
- **Nord** - Arctic, north-bluish color palette  
- **Dracula** - Dark theme with vibrant colors
- **TokyoNight** - Tokyo night lights inspired
- **OneDark** - Atom One Dark inspired
- **GruvboxDark** - Retro groove color scheme
- **Monokai** - Classic sublime colors
- **SolarizedDark** - Precision colors for machines and people

## Development

```bash
# Run in development
cargo run

# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt
```

## File Format

Todos are stored in a simple JSON format that's easy to read and version control:

```json
{
  "version": 1,
  "items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "title": "Example todo",
      "description": "- First task\n- Second task",
      "completed": false,
      "created_at": "2025-01-10T09:00:00Z",
      "completed_at": null
    }
  ]
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Created by [babinc](https://github.com/babinc)