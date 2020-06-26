use rand::thread_rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Card {
    UniZombie,
    ShiranuiSolitaire,
    Mezuki,
    Gozuki,
    NecroWorldBanshee,
    GlowUpBloom,
    ZombieWorld,
    Other,
}

fn generate_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(40);
    for _ in 0..3 {
        deck.push(Card::UniZombie);
        deck.push(Card::ShiranuiSolitaire);
        deck.push(Card::Mezuki);
        deck.push(Card::ZombieWorld);
    }
    for _ in 0..2 {
        deck.push(Card::Gozuki);
        deck.push(Card::NecroWorldBanshee);
    }
    for _ in 0..1 {
        deck.push(Card::GlowUpBloom);
    }
    while deck.len() < 40 {
        deck.push(Card::Other);
    }
    deck.shuffle(&mut thread_rng());
    deck
}

fn main() {
    println!("Hello, world!");
    let mut total_plays = HashMap::<PlayOptions, u64>::new();
    let runs = 10000;
    for _ in 0..runs {
        let deck = generate_deck();
        let plays = analyse(deck);
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    fn in_hand(&self, card: Card) -> bool {
        self.deck[..5].contains(&card)
    }

    fn in_deck(&self, card: Card) -> bool {
        self.deck[5..].contains(&card)
    }

    fn at_least_two_in_deck(&self, card: Card) -> bool {
        self.deck[5..].iter().filter(|&c| c == &card).count() >= 2
    }

    fn in_hand_or_deck(&self, card: Card) -> bool {
        self.deck.contains(&card)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum PlayOptions {
    Syncro,
    DoubleSyncro,
    DoomkingZombieWorld,
}

impl PlayOptions {
    fn all() -> Vec<PlayOptions> {
        vec![PlayOptions::Syncro, PlayOptions::DoubleSyncro, PlayOptions::DoomkingZombieWorld]
    }
}

fn analyse(deck: Vec<Card>) -> Vec<PlayOptions> {
    let deck = Deck { deck, };
    let mut plays = Vec::new();

    let solitaire_start = deck.in_hand(Card::ShiranuiSolitaire);
    let unizombie_still_in_deck = deck.in_deck(Card::UniZombie);
    let solitaire_into_unizombie = solitaire_start && unizombie_still_in_deck;

    let mill_gozuki_to_summon_mezuki = solitaire_into_unizombie
        && deck.in_deck(Card::Gozuki) && deck.in_hand(Card::Mezuki);
    let discard_gozuki_to_summon_mezuki = solitaire_into_unizombie
        && deck.in_hand(Card::Gozuki) && deck.in_hand(Card::Mezuki);
    // first syncro from unizombie and mezuki for level 8 (or a link 2)
    // then mezuki revives gozuki which gives a second mill to mill a second mezuki
    // which revives uni zombie for a level 7 syncro (or a link 2)
    let double_syncro = (mill_gozuki_to_summon_mezuki || discard_gozuki_to_summon_mezuki)
        && deck.at_least_two_in_deck(Card::Mezuki);
    if double_syncro {
        plays.push(PlayOptions::DoubleSyncro);
    }

    // uni zombie mills or discards mezuki to revivse soliaire and syncro for level 8 (or a link 2)
    let unizombie_revives_solitaire = solitaire_into_unizombie
        && deck.in_hand(Card::Mezuki) && deck.in_deck(Card::Mezuki);
    if unizombie_revives_solitaire {
        plays.push(PlayOptions::Syncro);
    }

    let unizombie_start = deck.in_hand(Card::UniZombie);
    let mezuki_banshee_mill = unizombie_start &&
        ((deck.in_hand(Card::Mezuki) && deck.in_deck(Card::NecroWorldBanshee))
        || (deck.in_deck(Card::Mezuki) && deck.in_hand(Card::NecroWorldBanshee)));
    // mezuki revives banshee, link summon needlefiber, special summon glow up bloom
    // activate banshee in grave, then summon something to send glow up bloom to grave
    // to special summon doomking
    let unizombie_into_doomking_zombie_world = mezuki_banshee_mill
        && deck.in_hand_or_deck(Card::GlowUpBloom) && deck.in_hand_or_deck(Card::ZombieWorld);

    // TODO: there are a lot of other ways to achieve this
    if unizombie_into_doomking_zombie_world {
        plays.push(PlayOptions::DoomkingZombieWorld)
    }

    plays
}
