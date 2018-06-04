#[derive(Clone, Copy)]
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

    pub fn to_left(self) -> Self {
        match self {
            Zone::Red => Zone::Red,
            Zone::White => Zone::Red,
            Zone::Blue => Zone::White,
        }
    }

    pub fn to_right(self) -> Self {
        match self {
            Zone::Red => Zone::White,
            Zone::White => Zone::Blue,
            Zone::Blue => Zone::Blue,
        }
    }
}

#[derive(Clone, Copy)]
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
