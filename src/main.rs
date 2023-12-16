use std::array;
use std::env;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum CamelCardHandRank {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

struct CamelCardHand {
    cards: [char; 5],
    bid: usize
}

impl CamelCardHand {

    fn hand_rank(&self) -> CamelCardHandRank {
        let card_ranks = String::from("J23456789TQKA");
        let mut counts: [u8; 14] = [0; 14];
        for card in self.cards.iter() {
            counts[card_ranks.find(*card).expect("Invalid rank!")] += 1;
        }

        let j_count = counts[0];  // transfer joker count to highest count
        counts[0] = 0;
        let max_index = counts
            .iter()
            .enumerate()
            .reduce(|a, b| if a.1 > b.1 {a} else {b})
            .unwrap().0;
        counts[max_index] += j_count;
        
        counts.sort();
        counts.reverse();
        match counts[0..5] {
            [5, 0, 0, 0, 0] => CamelCardHandRank::FiveOfAKind,
            [4, 1, 0, 0, 0] => CamelCardHandRank::FourOfAKind,
            [3, 2, 0, 0, 0] => CamelCardHandRank::FullHouse,
            [3, 1, 1, 0, 0] => CamelCardHandRank::ThreeOfAKind,
            [2, 2, 1, 0, 0] => CamelCardHandRank::TwoPair,
            [2, 1, 1, 1, 0] => CamelCardHandRank::Pair,
            _ => CamelCardHandRank::HighCard
        }
    }

}

fn main() {
    let path = env::args().nth(1).expect("Missing required parameter path!");

    let data: Vec<CamelCardHand> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(|line| {
            let text = line
                .expect("Could not read line!");
            let (cards, bid) = text
                .split_once(" ")
                .expect("Invalid line!");

            CamelCardHand {
                cards: cards.chars().into_iter().take(5).collect::<Vec<char>>().try_into().expect("Could not init array!"),
                bid: bid.parse::<usize>().expect("Invalid bid!")
            }

        })
        .collect();

    let mut rank_values: Vec<(CamelCardHandRank, usize, usize)> = data
        .into_iter().map(|h| (
            h.hand_rank(), 
            h.cards.iter().enumerate().map(|(i, c)| {
                let factor: usize = 100;
                let card_ranks = String::from("J23456789TQKA");
                card_ranks.find(*c).unwrap() * factor.pow(4 - i as u32)
            }).sum(),  // encode card ranks into a single usize
        h.bid))
        .collect();
    rank_values.sort();
    
    println!(
        "Total score: {}",
        rank_values.into_iter().enumerate().map(|(i, (_, _, bid))| (i + 1) * bid).sum::<usize>()
    )

}


#[test]
fn test_hand_rank() {

    let hand_1 = CamelCardHand {
        cards: ['A', 'A', 'A', 'A', 'A'], bid: 0
    };
    assert_eq!(hand_1.hand_rank(), CamelCardHandRank::FiveOfAKind);

    let hand_2 = CamelCardHand {
        cards: ['A', 'A', '8', 'A', 'A'], bid: 0
    };
    assert_eq!(hand_2.hand_rank(), CamelCardHandRank::FourOfAKind);

    let hand_3 = CamelCardHand {
        cards: ['2', '3', '3', '3', '2'], bid: 0
    };
    assert_eq!(hand_3.hand_rank(), CamelCardHandRank::FullHouse);

    let hand_4 = CamelCardHand {
        cards: ['T', 'T', 'T', '9', '8'], bid: 0
    };
    assert_eq!(hand_4.hand_rank(), CamelCardHandRank::ThreeOfAKind);

    let hand_5 = CamelCardHand {
        cards: ['2', '3', '4', '3', '2'], bid: 0
    };
    assert_eq!(hand_5.hand_rank(), CamelCardHandRank::TwoPair);

    let hand_6 = CamelCardHand {
        cards: ['A', '2', '3', 'A', '4'], bid: 0
    };
    assert_eq!(hand_6.hand_rank(), CamelCardHandRank::Pair);

    let hand_7 = CamelCardHand {
        cards: ['2', '3', '4', '5', '6'], bid: 0
    };
    assert_eq!(hand_7.hand_rank(), CamelCardHandRank::HighCard);

}


#[test]
fn test_ordering() {
    assert!(CamelCardHandRank::FiveOfAKind > CamelCardHandRank::FourOfAKind);
    assert!(CamelCardHandRank::FiveOfAKind > CamelCardHandRank::ThreeOfAKind);
    assert!(CamelCardHandRank::FiveOfAKind > CamelCardHandRank::HighCard);
    assert!(CamelCardHandRank::TwoPair > CamelCardHandRank::Pair);
    assert!(CamelCardHandRank::FourOfAKind > CamelCardHandRank::FullHouse);
    assert!(CamelCardHandRank::FullHouse > CamelCardHandRank::TwoPair);
}
