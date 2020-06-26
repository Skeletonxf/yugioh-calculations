use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Card {
    UniZombie,
    ShiranuiSolitaire,
    Mezuki,
    Gozuki,
    Other,
}

fn generate_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(40);
    for _ in 0..3 {
        deck.push(Card::UniZombie);
        deck.push(Card::ShiranuiSolitaire);
        deck.push(Card::Mezuki);
        deck.push(Card::Gozuki);
    }
    for _ in 0..(40-12) {
        deck.push(Card::Other);
    }
    deck.shuffle(&mut thread_rng());
    deck
}

fn main() {
    println!("Hello, world!");
    for _ in 0..5 {
        let deck = generate_deck();
        analyse(deck);
    }
}

fn analyse(deck: Vec<Card>) {
    println!("hand: {:?}", &deck[..5]);
}
