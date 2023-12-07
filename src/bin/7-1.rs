use std::io;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfKind,
	FullHouse,
	FourOfKind,
	FiveOfKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
	cards: [u8; 5],
}

impl Hand {
	pub fn parse(input: &str) -> Hand {
		assert!(input.len() == 5);
		let mut cards = [0u8; 5];
		for (i, c) in input.bytes().into_iter().enumerate() {
			cards[i] = match c {
				b'2' => 2,
				b'3' => 3,
				b'4' => 4,
				b'5' => 5,
				b'6' => 6,
				b'7' => 7,
				b'8' => 8,
				b'9' => 9,
				b'T' => 10,
				b'J' => 11,
				b'Q' => 12,
				b'K' => 13,
				b'A' => 14,
				_ => 0,
			};
		}
		return Hand { cards };
	}

	fn count(&self, card: u8) -> u8 {
		let mut count: u8 = 0;
		for c in self.cards {
			if c == card {
				count += 1;
			}
		}
		return count;
	}

	pub fn classify(&self) -> HandType {
		if self.count(self.cards[0]) == 5 {
			return HandType::FiveOfKind;
		} else if self.count(self.cards[0]) == 4 || self.count(self.cards[1]) == 4 {
			return HandType::FourOfKind;
		} else if self.cards.iter().any(|&c| self.count(c) == 3) {
			if self.cards.iter().any(|&c| self.count(c) == 2) {
				return HandType::FullHouse;
			} else {
				return HandType::ThreeOfKind;
			}
		} else if let Some(&paired) = self.cards.iter().find(|&&c| self.count(c) == 2) {
			if self
				.cards
				.iter()
				.any(|&c| c != paired && self.count(c) == 2)
			{
				return HandType::TwoPair;
			} else {
				return HandType::OnePair;
			}
		} else {
			return HandType::HighCard;
		}
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let self_type = self.classify();
		let other_type = other.classify();
		if self_type != other_type {
			return self_type.cmp(&other_type);
		} else {
			for (&sc, &oc) in self.cards.iter().zip(other.cards.iter()) {
				if sc > oc {
					return std::cmp::Ordering::Greater;
				} else if sc < oc {
					return std::cmp::Ordering::Less;
				}
			}
		}
		return std::cmp::Ordering::Equal;
	}
}

impl ToString for Hand {
	fn to_string(&self) -> String {
		let mut s = String::new();
		for c in self.cards {
			s += match c {
				2 => "2",
				3 => "3",
				4 => "4",
				5 => "5",
				6 => "6",
				7 => "7",
				8 => "8",
				9 => "9",
				10 => "T",
				11 => "J",
				12 => "Q",
				13 => "K",
				14 => "A",
				_ => "",
			};
		}
		return s;
	}
}

fn main() -> io::Result<()> {
	let iter = io::stdin().lines().map(|l| l.unwrap());
	let mut hands_and_bids: Vec<(Hand, u32)> = iter
		.map(|l| (Hand::parse(&l[0..5]), l[6..].parse().unwrap()))
		.collect();

	hands_and_bids.sort_by(|a, b| a.0.cmp(&b.0));

	let total_winnings: u32 = hands_and_bids
		.iter()
		.enumerate()
		.map(|(i, (_hand, bid))| {
			let rank = (i as u32) + 1;
			return (*bid) * rank;
		})
		.sum();

	dbg!(total_winnings);

	Ok(())
}
