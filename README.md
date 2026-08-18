# Just Solitaire 

A terminal-based game of Klondike built with Rust and the [ratatui](https://ratatui.rs/) TUI framework. Designed to be able to run on anything.

Most terminal games are unfortunately kind of ugly, or run slower than they feel like they should. I built Klondike with the idea that it should be lightweight, easy to look at, comfortable to use and run as fast on possible on any machine. 

### UI

**Includes:**
- Mouse & Keyboard controls 
- Drag and Drop functionality
- Autoclick
- Reactive UI

![Klondike Solitaire](assets/drag_and_drop.png)

### Controls

- Mouse: Click to select and drag cards
- Keyboard: Navigate with arrow keys, select with Enter/Space
- Press `?` or `h` for help menu
- Press `Esc` to open options menu

## Installation

### With bash or powershell

Linux / MacOS (or Windows with Git Bash/WSL), run in your terminal:

```
curl -sSfL https://raw.githubusercontent.com/marcus-wrrn/Terminal-Solitaire/main/install.sh | sh
```

Windows (PowerShell):

```
irm https://raw.githubusercontent.com/marcus-wrrn/Terminal-Solitaire/main/install.ps1 | iex
```

Afterwards all you need to do is run `solitaire` in your terminal.

## Download binary
Download the binary that supports your OS from the ![releases page](https://github.com/marcus-wrrn/Terminal-Solitaire/releases)

### With Cargo
First clone the repo and then build using: `cargo build`.



