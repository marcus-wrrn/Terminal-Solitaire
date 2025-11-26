mod game_objects;
mod rendering;

use game_objects::Deck;
use game_objects::{Pile, PileType};
use rendering::CardRenderer;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

fn main() -> Result<(), io::Error> {
    let mut deck = Deck::new();
    deck.shuffle();

    println!("Created and shuffled a deck of {} cards", deck.len());

    // let mut tableau_pile = Pile::new(PileType::Tableau);
    // let mut foundation_pile = Pile::new(PileType::Foundation);
    let mut stock_pile = Pile::new(PileType::Stock);
    // let mut waste_pile = Pile::new(PileType::Waste);

    let drawn_cards = deck.draw_multiple(5);
    for card in drawn_cards {
        stock_pile.push(card);
    }

    println!("Stock pile has {} cards", stock_pile.len());
    println!("Backend card and pile logic initialized successfully!");

    let mut random_card = deck.draw().expect("Failed to draw a card");
    random_card.flip();

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    terminal.draw(|frame| {
        let area = frame.area();

        let card_renderer = CardRenderer::new(
            random_card,
            (5, 3),
            10,
            7,
        );

        frame.render_widget(card_renderer, area);
    })?;

    std::thread::sleep(std::time::Duration::from_secs(3));

    terminal.clear()?;

    Ok(())
}
