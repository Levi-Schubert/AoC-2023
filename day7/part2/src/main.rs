use std::fs;
use std::cmp::Ordering;
use std::error::Error;
use std::time::SystemTime;


fn main() -> Result<(), Box<dyn Error>>{
    let start_time = SystemTime::now();
    let input: String = fs::read_to_string("input/input.txt")?.parse()?;
    let lines: Vec<_> = input.split("\r\n").collect();

    let mut hands = Vec::<Hand>::new();

    for line in lines{
        hands.push(parse_line(line));
    }

    // for hand in &hands{
    //     println!("{:?}", hand);
    // }
    // println!("^hand pre-sort|hand post-sort v");

    hands.sort();
    for hand in &hands{
        println!("{:?}", hand);
    }

    let mut rank = 1;
    let mut sum = 0;
    for hand in hands{
        sum += rank * hand.bid;
        rank += 1;
    }

    let end_time = SystemTime::now();
    let time_taken = end_time.duration_since(start_time).expect("Clock may have gone backwards");
    println!("answer: {} \ntime taken: {:?}", sum, time_taken);
    Ok(())
}

fn parse_line(line: &str) -> Hand {
    let input: Vec<_> = line.split(" ").collect();
    let cards = parse_cards(input[0]);
    let bid = parse_bid(input[1]);
    let hand_type = get_hand_type(&cards);

    Hand{
        cards: cards.clone(),
        bid: bid,
        hand_type: hand_type,
    }
}

fn parse_cards(line: &str) -> Vec<Card>{
    let mut cards = Vec::<Card>::new();
    for ch in line.chars(){
        cards.push(Card{
            face: ch
        });
    }
    return cards;
}

fn parse_bid(line: &str) -> i32{
    return line.parse::<i32>().unwrap();
}

fn get_hand_type(cards: &Vec<Card>) -> HandType{
    let mut card_types:Vec<(Card, i8)> = Vec::<(Card, i8)>::new();
    let mut has_joker = false;

    for card in cards{
        if card.face == 'J'{
            has_joker = true;
        }
        if card_types.len() == 0{
            card_types.push((card.clone(), 1));
        }else{
            if let Some(c) = card_types.iter().position(|&c| c.0 == card.clone()){
                card_types[c].1 += 1;
            }else{
                card_types.push((card.clone(), 1));
            }
        }
    }

    if !has_joker{
        if card_types.len() == 1{
            return HandType::FiveOfAKind;
        } 
        if card_types.len() == 2{
            if card_types[0].1 == 2 || card_types[1].1 == 2 {
                return HandType::FullHouse;
            }
            return HandType::FourOfAKind;
        }
        if card_types.len() == 3{
            if card_types[0].1 == 3 || card_types[1].1 == 3 || card_types[2].1 == 3 {
                return HandType::ThreeOfAKind;
            }
            return HandType::TwoPair;
        }
        if card_types.len() == 4 {
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }else{
        let joker_count = card_types[card_types.iter().position(|&c| c.0.face == 'J').unwrap()].1;
        if joker_count == 5 || joker_count == 4{
            return HandType::FiveOfAKind;
        }
        if joker_count == 3{
            if card_types.len() == 2 {
                return HandType::FiveOfAKind;
            }
            return HandType::FourOfAKind;
        }
        if joker_count == 2 {
            if card_types.len() == 2{
                return HandType::FiveOfAKind;
            }
            if card_types.len() == 3{
                return HandType::FourOfAKind;
            }
            return HandType::ThreeOfAKind;
        }
        if card_types.len() == 2{
            return HandType::FiveOfAKind;
        }
        if card_types.len() == 3{
            if card_types[0].1 == 3 || card_types[1].1 == 3 || card_types[2].1 == 3{
                return HandType::FourOfAKind;
            }
            if card_types[0].1 == 2 || card_types[1].1 == 2 || card_types[2].1 == 2{
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        } 
        if card_types.len() == 4 {
            if card_types[0].1 == 2 || card_types[1].1 == 2 || card_types[2].1 == 2 || card_types[3].1 == 2{
                return HandType::ThreeOfAKind;
            }
            return HandType::OnePair;
        }
        return HandType::OnePair;
    }
}


#[derive(Debug, Eq)]
struct Hand{
    cards: Vec<Card>,
    hand_type: HandType,
    bid: i32
}

impl Ord for Hand{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.value().cmp(&other.hand_type.value());
        };
        let mut count = 0;
        for card in self.cards.clone(){
            if card != other.cards[count]{
                return card.cmp(&other.cards[count])
            }
            count += 1
        }
        self.cards.cmp(&other.cards)   
    }
}

impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand{
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum HandType{
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType{
    fn value(&self) -> i32{
        match *self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }

    fn eq(&self, other: &Self) -> bool{
        return self.value() == other.value()
    }
}

#[derive(Debug, Eq, Copy, Clone)]
struct Card{
    face: char
}

impl Ord for Card{
    fn cmp(&self, other: &Self) -> Ordering {
        let card_strength_map = vec!['0', 'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

        let card_one = card_strength_map.iter().position(|&c| c == self.face).unwrap(); 
        let card_two = card_strength_map.iter().position(|&c| c == other.face).unwrap(); 
        card_one.cmp(&card_two)   
    }
}

impl PartialOrd for Card{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.cmp(other))
    }
}

impl PartialEq for Card{
    fn eq(&self, other: &Self) -> bool {
        self.face == other.face
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_one(){
//         let result = parse_line("");
//         assert_eq!(result, "result");
//     }
// }