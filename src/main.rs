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
        deck.push(Card::Downbeat);
        deck.push(Card::TenyiSpiritAdhara);
    }
    for _ in 0..2 {
        deck.push(Card::DoomkingBalerdroch);
        deck.push(Card::NecroWorldBanshee);
        deck.push(Card::Gozuki);
        deck.push(Card::GhostBelleAndHauntedMansion);
        deck.push(Card::SamuraiSkull);
        //deck.push(Card::JackOBolan);
    }
    for _ in 0..1 {
        //deck.push(Card::GoblinZombie);
        //deck.push(Card::ZombieMaster);
        deck.push(Card::GlowUpBloom);
        deck.push(Card::ShiranuiSpiritmaster);
        deck.push(Card::ShiranuiSpectralsword);
        deck.push(Card::CardDestruction);
        deck.push(Card::UpstartGoblin);
    }
    while deck.len() < 40 {
        deck.push(Card::Other);
    }
    deck.shuffle(&mut thread_rng());
    GameState::from(deck)
}

fn main() {
    let mut total_plays = HashMap::<PlayOptions, u64>::new();
    let runs = 50000;
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
    SummonUniZombie,
}

impl PlayOptions {
    fn all() -> Vec<PlayOptions> {
        vec![
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

/**
 * Attempts to use Jack 'o Bolan's discard to summon effect to get unizombie
 * out from the deck via Needlefiber and a normal summoned tuner (ie not the hand).
 */
fn jackobolan_and_tuner_into_unizombie(game: GameState) -> Option<GameState> {
    game.clone().summon_from_hand(Card::GlowUpBloom)
        .or_else(|| game.clone().summon_from_hand(Card::ShiranuiSpectralsword))
        .or_else(|| game.summon_from_hand(Card::GhostBelleAndHauntedMansion))
        // mezuki seems to be the only discard that can revive unizombie
        // from the grave
        .and_then(|game| game.discard(Card::Mezuki))
        .and_then(|game| game.summon_from_hand(Card::JackOBolan))
        // summon Needlefiber
        .and_then(|game| game.send_to_grave(Card::JackOBolan))
        .and_then(|game| game.clone().send_to_grave(Card::GlowUpBloom)
            .or_else(|| game.clone().send_to_grave(Card::ShiranuiSpectralsword))
            .or_else(|| game.send_to_grave(Card::GhostBelleAndHauntedMansion))
        )
        .and_then(|game| game.summon_from_extra_deck(Card::Link2))
        // revive unizombie with the discarded mezuki
        .and_then(|game| game.banish_from_grave(Card::Mezuki))
        .and_then(|game| game.summon_from_grave(Card::UniZombie))
}

/**
 * Attempts to use Downbeat on a level 4 DARK zombie monster to bring out unizombie
 *
 * This raises the chances of summoning unizombie from 57% with just unizombie and soliaire
 * to 69% with a bunch of level 4 DARK zombies.
 */
fn downbeat_into_unizombie(game: GameState) -> Option<GameState> {
    game.clone().summon_from_hand(Card::SamuraiSkull)
        .or_else(|| game.clone().summon_from_hand(Card::ZombieMaster))
        .or_else(|| game.clone().summon_from_hand(Card::NecroWorldBanshee))
        .or_else(|| game.summon_from_hand(Card::GoblinZombie))
        .and_then(|game| game.activate(Card::Downbeat))
        .and_then(|game| game.summon_from_deck(Card::UniZombie))
        .and_then(|game| game.clone().send_to_grave(Card::SamuraiSkull)
            .or_else(|| game.clone().send_to_grave(Card::ZombieMaster))
            .or_else(|| game.clone().send_to_grave(Card::NecroWorldBanshee))
            .or_else(|| game.send_to_grave(Card::GoblinZombie))
        )
}

/**
 * Attempts to use Jack 'o Bolan and Downbeat on a level 4 EARTH zombie monster
 * to bring out Ghost Belle to use Crystron Needlefiber to search unizombie and
 * then revive it from the grave to unnegate it.
 *
 * Even with both of these cards set to 3 the probability of summoning unizombie
 * only goes up from 67 to 70 percent compared to running 0 Jack 'o Bolans
 */
fn jackobolan_and_downbeat_into_unizombie(game: GameState) -> Option<GameState> {
    // actual card effect is discard for cost to summon but testing that
    // the card is in hand first short circuits on the fail case faster
    game.summon_from_hand(Card::JackOBolan)
        // the discard isn't actually important for this method, so
        // any zombie which isn't a level 4 DARK monster or unizombie
        // or Shiranui Solitaire is included
        .and_then(|game| game.clone().discard(Card::DoomkingBalerdroch)
            .or_else(|| game.clone().discard(Card::GlowUpBloom))
            .or_else(|| game.clone().discard(Card::ShiranuiSpectralsword))
            .or_else(|| game.clone().discard(Card::ShiranuiSpiritmaster))
            .or_else(|| game.clone().discard(Card::GhostBelleAndHauntedMansion))
            .or_else(|| game.clone().discard(Card::Mezuki))
            .or_else(|| game.discard(Card::Gozuki))
        )
        // summon mezuki or summon gozuki and mill mezuki
        .and_then(|game| game.clone().summon_from_hand(Card::Mezuki)
            .or_else(|| game.summon_from_hand(Card::Gozuki)
                .and_then(|game| {
                    let _game = game.clone().mill_to_grave(Card::Mezuki);
                    // even if no mezukis for gozuki to mill, if discarded
                    // one via jackaboolan already then can continue combo
                    match _game {
                        Some(g) => Some(g),
                        None => {
                            if game.in_grave(Card::Mezuki) {
                                Some(game)
                            } else {
                                None
                            }
                        }
                    }
                })
            )
        )
        .and_then(|game| game.activate(Card::Downbeat))
        .and_then(|game| game.clone().send_to_grave(Card::Mezuki)
            .or_else(|| game.send_to_grave(Card::Gozuki))
        )
        .and_then(|game| game.summon_from_deck(Card::GhostBelleAndHauntedMansion))
        // Crystron Needlefiber can search unizombie (albeit negated)
        .and_then(|game| game.send_to_grave(Card::JackOBolan))
        .and_then(|game| game.send_to_grave(Card::GhostBelleAndHauntedMansion))
        .and_then(|game| game.summon_from_extra_deck(Card::Link2))
        .and_then(|game| game.summon_from_deck(Card::UniZombie))
        // summon any link 2 to send the uni zombie to grave
        .and_then(|game| game.send_to_grave(Card::Link2))
        .and_then(|game| game.send_to_grave(Card::UniZombie))
        .and_then(|game| game.summon_from_extra_deck(Card::Link2))
        // use the mezuki placed into grave earlier to revive the unizombie
        .and_then(|game| game.banish_from_grave(Card::Mezuki))
        .and_then(|game| game.summon_from_grave(Card::UniZombie))
}

/**
 * Attempts to use Tenyi Spirit Adhara and a normal summon to bring out Needlefiber
 * with a way to revive the Uni Zombie summoned off Needlefiber
 */
fn adhara_into_unizombie(game: GameState) -> Option<GameState> {
    game.summon_from_hand(Card::TenyiSpiritAdhara)
        .and_then(|game| game.clone().summon_from_hand(Card::Mezuki)
            .or_else(|| game.clone().summon_from_hand(Card::Gozuki)
                .and_then(|game| game.mill_to_grave(Card::Mezuki))
            )
            .or_else(|| game.summon_from_hand(Card::SamuraiSkull)
                .and_then(|game| game.mill_to_grave(Card::Mezuki))
            )
        )
        // summon needlefiber
        .and_then(|game| game.send_to_grave(Card::TenyiSpiritAdhara))
        .and_then(|game| game.clone().send_to_grave(Card::Mezuki)
            .or_else(|| game.clone().send_to_grave(Card::Gozuki))
            .or_else(|| game.send_to_grave(Card::SamuraiSkull))
        )
        .and_then(|game| game.summon_from_extra_deck(Card::Link2))
        .and_then(|game| game.summon_from_deck(Card::UniZombie))
        // it doesn't matter what method is used to get uni zombie
        // into the grave, so assume another link 2 monster
        .and_then(|game| game.send_to_grave(Card::Link2))
        .and_then(|game| game.send_to_grave(Card::UniZombie))
        .and_then(|game| game.summon_from_extra_deck(Card::Link2))
        .and_then(|game| game.banish_from_grave(Card::Mezuki))
        .and_then(|game| game.summon_from_grave(Card::UniZombie))
}

/**
 * Attempts to use card destruction to refresh the hand and summon unizombie
 */
fn card_destruction_into_unizombie(game: GameState) -> Option<GameState> {
    // try setting downbeat if its in the hand before playing card destruction
    game.clone().set(Card::Downbeat)
        .or_else(|| Some(game))
        .and_then(|game| game.activate(Card::CardDestruction))
        .and_then(|game| game.shuffle_deck())
        .and_then(|game| {
            let cards = game.hand.len();
            game.discard_hand().and_then(|game| game.draw(cards))
        })
        .and_then(|game| game.clone().return_to_hand(Card::Downbeat)
            .or_else(|| Some(game))
        )
        .and_then(|game| unizombie_from_hand(game.clone())
            .or_else(|| solitaire_into_unizombie(game.clone()))
            .or_else(|| jackobolan_into_unizombie(game.clone()))
            .or_else(|| downbeat_into_unizombie(game.clone()))
            .or_else(|| jackobolan_and_downbeat_into_unizombie(game.clone()))
            .or_else(|| jackobolan_and_tuner_into_unizombie(game.clone()))
            .or_else(|| adhara_into_unizombie(game))
        )
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
    match downbeat_into_unizombie(game.clone()) {
        Some(game) => methods.push(game),
        None => (),
    };
    match jackobolan_and_downbeat_into_unizombie(game.clone()) {
        Some(game) => methods.push(game),
        None => (),
    };
    match jackobolan_and_tuner_into_unizombie(game.clone()) {
        Some(game) => methods.push(game),
        None => (),
    };
    match adhara_into_unizombie(game.clone()) {
        Some(game) => methods.push(game),
        None => (),
    };
    match card_destruction_into_unizombie(game.clone()) {
        Some(game) => methods.push(game),
        None => (),
    };

    // foolish burial / monster reborn + samurai skull / gozuki

    methods
}

fn analyse(mut game: GameState) -> Vec<PlayOptions> {
    let mut plays = Vec::new();

    if game.in_hand(Card::UpstartGoblin) {
        game = game.activate(Card::UpstartGoblin)
            .and_then(|game| game.draw(1)).unwrap()
    }

    if !can_summon_unizombie(game.clone()).is_empty() {
        plays.push(PlayOptions::SummonUniZombie);
    }
    plays
}
