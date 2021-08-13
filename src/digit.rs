use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl fmt::Display for Digit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let digit = match self {
			Digit::One => 1,
			Digit::Two => 2,
			Digit::Three => 3,
			Digit::Four => 4,
			Digit::Five => 5,
			Digit::Six => 6,
			Digit::Seven => 7,
			Digit::Eight => 8,
			Digit::Nine => 9,
		};
		write!(f, "{}", digit)
	}
}
