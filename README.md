# Yu Gi Oh Calculator

This is a Rust Yu Gi Oh calculator that uses a very simplified game state simulator to calculate approximate probabilities of whatever you code it to.

I'm using it for computing probabilities with Zombie decks, but the game state representation code can be used for more general purposes and is very flexible for exploring plays, as you can make extensive use of `Option`s `and_then` and `or_else` to attempt plays and branch on different cards. Rather than explictly code in edge cases like having 3 copies of a card in your hand and thus not being able to send it from your deck to grave, this rough simulation approach allows you to just try various steps and propagate the successes and failures without explicit edge case handling.

```rust
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
```

Because the cards are representated as enums (and consequently just numbers) this approach is very efficient. I can run 50000 hands to explore half a dozen opening strategies in less than 3 seconds with code that is not optimised at all.

*****

YDK format is quite common in YuGiOh deck editors as an import/export format and I don't think it would be very hard to add limited support for a small subset of YuGiOh cards. However, the exploration of different strategies requires manually coding in the steps to take with regards to each explicit card name, so I've not opted to implement any file parsing that could be used to analyse/compare multiple deck lists. This code can be repurposed as is by changing the strategies in `main.rs` and decklist defined `generate_game`.
