use rand::thread_rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

mod representation;

use representation::{Card, GameState};

fn generate_game() -> GameState {
    let mut deck = Vec::with_capacity(40);
    for _ in 0..3 {
        deck.push(Card::UniZombie);
        deck.push(Card::ShiranuiSolitaire);
        deck.push(Card::Mezuki);
        deck.push(Card::ZombieWorld);
    }
    for _ in 0..2 {
        deck.push(Card::NecroWorldBanshee);
        deck.push(Card::JackOBolan);
        deck.push(Card::Gozuki);
    }
    for _ in 0..1 {
        deck.push(Card::GlowUpBloom);
    }
    while deck.len() < 40 {
        deck.push(Card::Other);
    }
    deck.shuffle(&mut thread_rng());
    GameState::from(deck)
}

fn main() {
    println!("Hello, world!");
    let mut total_plays = HashMap::<PlayOptions, u64>::new();
    let runs = 10000;
    for _ in 0..runs {
        let game = generate_game();
        let plays = analyse(game);
        for play in plays {
            let new_count = {
                match total_plays.get(&play) {
                    Some(count) => count + 1,
                    None => 1,
                }
            };
            total_plays.insert(play, new_count);
        }
    }
    for play in &PlayOptions::all() {
        println!("{:?}: {:.1}%", play, (*total_plays.get(play).unwrap_or(&0) as f64) / (runs as f64) * 100.0);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum PlayOptions {
    DoomkingZombieWorld,
    SummonUniZombie,
}

impl PlayOptions {
    fn all() -> Vec<PlayOptions> {
        vec![
            PlayOptions::DoomkingZombieWorld,
            PlayOptions::SummonUniZombie,
        ]
    }
}

/**
 * Attempts to summon soliaire from hand and special summon unizombie from deck
 *
 * This would fail if for example all the unizombies were already in the hand,
 * in which case None is returned indicating faliure. On success the resultant
 * game state is returned to test for further combos.
 */
fn solitaire_into_unizombie(game: GameState) -> Option<GameState> {
    game.summon_from_hand(Card::ShiranuiSolitaire)
        .and_then(|game| game.send_to_grave(Card::ShiranuiSolitaire))
        .and_then(|game| game.summon_from_deck(Card::UniZombie))
}

/**
 * Attempts to summon unizombie from the hand.
 *
 * This would obviously fail if unizombie wasn't in the hand.
 */
fn unizombie_from_hand(game: GameState) -> Option<GameState> {
    game.summon_from_hand(Card::UniZombie)
}

/**
 * Attempts to use Jack 'o Bolan's discard to summon effect to get unizombie
 * out from the deck (ie not the hand).
 */
fn jackobolan_into_unizombie(game: GameState) -> Option<GameState> {
    // actual card effect is discard for cost to summon but testing that
    // the card is in hand first short circuits on the fail case faster
    game.summon_from_hand(Card::JackOBolan)
        // mezuki seems to be the only discard aside that can pull unizombie
        // out of the deck ignoring the obvious unizombie/solitaire discards
        .and_then(|game| game.discard(Card::Mezuki))
        // summoned jackaboolan by discarding something,
        // now normal summon something that can mill unizombie
        .and_then(|game| game.clone().summon_from_hand(Card::SamuraiSkull)
            .or_else(|| game.summon_from_hand(Card::Gozuki))
        )
        .and_then(|game| game.mill_to_grave(Card::UniZombie))
        .and_then(|game| game.banish_from_grave(Card::Mezuki))
        .and_then(|game| game.summon_from_grave(Card::UniZombie))
}

fn can_summon_unizombie(game: GameState) -> Vec<GameState> {
    let mut methods = vec![];

    match unizombie_from_hand(game.clone()) {
        Some(game) => methods.push(game),
        None => (),
    };
    match solitaire_into_unizombie(game.clone()) {
        Some(game) => methods.push(game),
        None => (),
    };
    match jackobolan_into_unizombie(game.clone()) {
        Some(game) => methods.push(game),
        None => (),
    };

    methods
}

fn analyse(game: GameState) -> Vec<PlayOptions> {
    let mut plays = Vec::new();

    if !can_summon_unizombie(game.clone()).is_empty() {
        plays.push(PlayOptions::SummonUniZombie);
    }
    plays
}
