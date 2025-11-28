# Solitaire Project Documentation

## Source Files

### Main Entry Point
- `src/main.rs` - Entry point that initializes deck, piles, and tests basic game logic
- `src/game_manager.rs` - Main Game object for controlling the state + mechanics of the game

### Game Objects Module (`src/game_objects/`)
- `mod.rs` - Module declaration file that re-exports Card, Deck, Pile, and related types
- `card.rs` - Defines Card struct with Suit and Rank enums, color checking, and display formatting
- `deck.rs` - Standard n-card deck implementation with shuffle and draw functionality
- `pile.rs` - Pile struct with placement rules for Tableau, Foundation, Stock, and Waste piles
- `board.rs` - Board struct that contains all piles + cards needed for solitaire

### Rendering Module (`src/rendering`)
- `board_renderer.rs` - Renders the full board
- `pile_renderer.rs` - Renders cards in a pile
- `card_renderer.rs` - Renders single Card structs

### Controller Module (`src/controller`)
- `controller.rs`   - Controller Structure
- `key_bindings.rs` - Rebindable keybinds used by Controller

## Dependencies (from Cargo.toml)

- `ratatui` (v0.29.0) - Terminal UI library for building text-based user interfaces
- `rand` (v0.8)
- `crossterm` (v0.28)

## Tool Usage

Reference the ratatui documentation at https://ratatui.rs/ (high level tutorial) or https://docs.rs/ratatui/latest/ratatui/ (Rust Docs) when developing a new feature with no working examples in the project