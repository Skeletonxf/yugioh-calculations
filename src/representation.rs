#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Card {
    UniZombie,
    ShiranuiSolitaire,
    Mezuki,
    Gozuki,
    NecroWorldBanshee,
    GlowUpBloom,
    ZombieWorld,
    JackOBolan,
    SamuraiSkull,
    ZombieMaster,
    GoblinZombie,
    Other,
    Downbeat,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameState {
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
    pub field: Vec<Card>,
    pub grave: Vec<Card>,
    pub banished: Vec<Card>,
}

impl GameState {
    pub fn from(deck: Vec<Card>) -> GameState {
        GameState {
            hand: deck[..5].to_vec(),
            deck: deck[5..].to_vec(),
            field: vec![],
            grave: vec![],
            banished: vec![],
        }
    }

    // pub fn in_hand(&self, card: Card) -> bool {
    //     self.hand.contains(&card)
    // }
    //
    // pub fn in_deck(&self, card: Card) -> bool {
    //     self.deck.contains(&card)
    // }
    //
    // pub fn on_field(&self, card: Card) -> bool {
    //     self.field.contains(&card)
    // }
    //
    // pub fn in_grave(&self, card: Card) -> bool {
    //     self.grave.contains(&card)
    // }
    //
    // pub fn in_hand_or_deck(&self, card: Card) -> bool {
    //     self.in_hand(card) || self.in_deck(card)
    // }
    //
    // pub fn at_least_two_in_deck(&self, card: Card) -> bool {
    //     self.deck.iter().filter(|&c| c == &card).count() >= 2
    // }

    pub fn summon_from_hand(mut self, card: Card) -> Option<GameState> {
        self.hand.remove(self.hand.iter().position(|&c| c == card)?);
        self.field.push(card);
        Some(self)
    }

    pub fn summon_from_deck(mut self, card: Card) -> Option<GameState> {
        self.deck.remove(self.deck.iter().position(|&c| c == card)?);
        self.field.push(card);
        Some(self)
    }

    pub fn send_to_grave(mut self, card: Card) -> Option<GameState> {
        self.field.remove(self.field.iter().position(|&c| c == card)?);
        self.grave.push(card);
        Some(self)
    }

    pub fn mill_to_grave(mut self, card: Card) -> Option<GameState> {
        self.deck.remove(self.deck.iter().position(|&c| c == card)?);
        self.grave.push(card);
        Some(self)
    }

    pub fn discard(mut self, card: Card) -> Option<GameState> {
        self.hand.remove(self.hand.iter().position(|&c| c == card)?);
        self.grave.push(card);
        Some(self)
    }

    pub fn activate(self, card: Card) -> Option<GameState> {
        self.discard(card)
    }

    // pub fn summon_from_extra_deck(mut self, card: Card) -> Option<GameState> {
    //     self.field.push(card);
    //     Some(self)
    // }

    pub fn summon_from_grave(mut self, card: Card) -> Option<GameState> {
        self.grave.remove(self.grave.iter().position(|&c| c == card)?);
        self.field.push(card);
        Some(self)
    }

    pub fn banish_from_grave(mut self, card: Card) -> Option<GameState> {
        self.grave.remove(self.grave.iter().position(|&c| c == card)?);
        self.banished.push(card);
        Some(self)
    }
}
