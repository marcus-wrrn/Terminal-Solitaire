# Solitaire Project Documentation

## Dev notes

Do not abuse comments and use the UNIX philosophy

## Source Files

### Main Entry Point
- `src/main.rs` - Entry point that initializes the game and starts the main loop

### Game Objects Module (`src/game_objects/`)
Pure data structures representing game entities. This module has no dependencies on game logic or rendering.

- `mod.rs` - re-exports all game object types
- `card.rs` - Defines Card struct with Suit and Rank enums, color checking, and display formatting
- `deck.rs` - Standard n-card deck implementation with shuffle and draw functionality
- `pile.rs` - Pile struct with placement rules for Tableau, Foundation, Stock, and Waste piles
- `board.rs` - Board struct that contains all piles and cards for solitaire (pure data structure)
- `selection.rs` - Selection struct representing a position on the board (pile type, pile index, card index)
- `tests.rs` - Unit tests for Card, Deck, and Pile structs

### Game Logic Module (`src/game_logic/`)
Orchestrates game state and manages game mechanics. Depends on game_objects and rendering but not vice versa.

- `mod.rs` - re-exports GameState, GameManager, SelectionNavigator, AnimationManager, HoverState, MenuManager, and MenuAction
- `game_state.rs` - GameState struct that holds the board and current selection, handles selection state updates
- `game_manager.rs` - GameManager struct that runs the main game loop, processes input, and coordinates rendering
- `selection_navigator.rs` - SelectionNavigator handles keyboard-based navigation and movement of the selection cursor across different pile types
- `animation_manager.rs` - AnimationManager handles win animations and auto-play sequences when game is won or all tableau cards are face up
- `hover_state.rs` - HoverState enum representing mouse hover states (Valid, Invalid, or None)
- `menu_manager.rs` - MenuManager coordinates the options menu and victory screen, processes menu actions

### Rendering Module (`src/rendering`)
Renders game objects 

Rendering Pipeline GameRenderer->BoardRenderer->PileRenderer->CardRenderer

- `mod.rs` - re-exports GameRenderer and BoardRenderer
- `game_renderer.rs` - Main game renderer that coordinates title, board, and debug log display
- `board_renderer.rs` - Renders the full board
- `pile_renderer.rs` - Renders cards, determines if card to be rendered is selected
- `card_renderer.rs` - Renders single Card

### Controller Module (`src/controller`)
Handles game input

- `mod.rs` - re-exports KeyBindings, Controller, and GameAction
- `controller.rs` - Controller struct that handles user input (both mouse + keyboard) and maps to GameActions
- `key_bindings.rs` - Rebindable keybinds used by Controller

### UI Module (`src/ui`)
- `mod.rs` - re-exports DebugLog, OptionsMenu, MenuOption, and WinPopup
- `debug.rs` - DebugLog component for displaying debug messages in the UI
- `options_menu.rs` - OptionsMenu widget displaying game options (Restart, Rebind Keys, Help, Developer Mode)
- `popups/` - Submodule containing specialized popup screens
  - `mod.rs` - re-exports VictoryScreen
  - `victory_popup.rs` - VictoryScreen widget displayed when the player wins the game

## Dependencies (from Cargo.toml)

- `ratatui` (v0.29.0) - Terminal UI library for building text-based user interfaces
- `rand` (v0.8)
- `crossterm` (v0.28)

## Tool Usage

Reference the ratatui documentation at https://ratatui.rs/ (high level tutorial) or https://docs.rs/ratatui/latest/ratatui/ (Rust Docs) when developing a new feature with no working examples in the project