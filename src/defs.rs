#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Zone {
    Red,
    White,
    Blue,
}

impl Zone {
    pub fn flip(self) -> Self {
        match self {
            Zone::Red => Zone::Blue,
            Zone::Blue => Zone::Red,
            Zone::White => Zone::White,
        }
    }

    pub fn left(self) -> Self {
        match self {
            Zone::Red => Zone::Red,
            Zone::White => Zone::Red,
            Zone::Blue => Zone::White,
        }
    }

    pub fn right(self) -> Self {
        match self {
            Zone::Red => Zone::White,
            Zone::White => Zone::Blue,
            Zone::Blue => Zone::Blue,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Deck {
    Upper,
    Lower,
}

impl Deck {
    pub fn flip(self) -> Self {
        match self {
            Deck::Upper => Deck::Lower,
            Deck::Lower => Deck::Upper,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Room {
    zone: Zone,
    deck: Deck,
}

impl Room {
    pub fn left(self) -> Self {
        Room {
            zone: self.zone.left(),
            deck: self.deck,
        }
    }

    pub fn right(self) -> Self {
        Room {
            zone: self.zone.right(),
            deck: self.deck,
        }
    }

    pub fn h_flip(self) -> Self {
        Room {
            zone: self.zone.flip(),
            deck: self.deck,
        }
    }

    pub fn v_flip(self) -> Self {
        Room {
            zone: self.zone,
            deck: self.deck.flip(),
        }
    }

    pub fn x_flip(self) -> Self {
        Room {
            zone: self.zone.flip(),
            deck: self.deck.flip(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Subsystem {
    A,
    B,
    C,
}
