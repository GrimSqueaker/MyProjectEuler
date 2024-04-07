// <p>In the card game poker, a hand consists of five cards and are ranked, from lowest to highest, in the following way:</p>
// <ul><li><b>High Card</b>: Highest value card.</li>
// <li><b>One Pair</b>: Two cards of the same value.</li>
// <li><b>Two Pairs</b>: Two different pairs.</li>
// <li><b>Three of a Kind</b>: Three cards of the same value.</li>
// <li><b>Straight</b>: All cards are consecutive values.</li>
// <li><b>Flush</b>: All cards of the same suit.</li>
// <li><b>Full House</b>: Three of a kind and a pair.</li>
// <li><b>Four of a Kind</b>: Four cards of the same value.</li>
// <li><b>Straight Flush</b>: All cards are consecutive values of same suit.</li>
// <li><b>Royal Flush</b>: Ten, Jack, Queen, King, Ace, in same suit.</li>
// </ul><p>The cards are valued in the order:<br>2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.</p>
// <p>If two players have the same ranked hands then the rank made up of the highest value wins; for example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie, for example, both players have a pair of queens, then highest cards in each hand are compared (see example 4 below); if the highest cards tie then the next highest cards are compared, and so on.</p>
// <p>Consider the following five hands dealt to two players:</p>
// <div class="center">
// <table><tr><td><b>Hand</b></td><td> </td><td><b>Player 1</b></td><td> </td><td><b>Player 2</b></td><td> </td><td><b>Winner</b></td>
// </tr><tr><td><b>1</b></td><td> </td><td>5H 5C 6S 7S KD<br><div class="smaller">Pair of Fives</div></td><td> </td><td>2C 3S 8S 8D TD<br><div class="smaller">Pair of Eights</div></td><td> </td><td>Player 2</td>
// </tr><tr><td><b>2</b></td><td> </td><td>5D 8C 9S JS AC<br><div class="smaller">Highest card Ace</div></td><td> </td><td>2C 5C 7D 8S QH<br><div class="smaller">Highest card Queen</div></td><td> </td><td>Player 1</td>
// </tr><tr><td><b>3</b></td><td> </td><td>2D 9C AS AH AC<br><div class="smaller">Three Aces</div></td><td> </td><td>3D 6D 7D TD QD<br><div class="smaller">Flush  with Diamonds</div></td><td> </td><td>Player 2</td>
// </tr><tr><td><b>4</b></td><td> </td><td>4D 6S 9H QH QC<br><div class="smaller">Pair of Queens<br>Highest card Nine</div></td><td> </td><td>3D 6D 7H QD QS<br><div class="smaller">Pair of Queens<br>Highest card Seven</div></td><td> </td><td>Player 1</td>
// </tr><tr><td><b>5</b></td><td> </td><td>2H 2D 4C 4D 4S<br><div class="smaller">Full House<br>With Three Fours</div></td><td> </td><td>3C 3D 3S 9S 9D<br><div class="smaller">Full House<br>with Three Threes</div></td><td> </td><td>Player 1</td>
// </tr></table></div>
// <p>The file, <a href="resources/documents/0054_poker.txt">poker.txt</a>, contains one-thousand random hands dealt to two players. Each line of the file contains ten cards (separated by a single space): the first five are Player 1's cards and the last five are Player 2's cards. You can assume that all hands are valid (no invalid characters or repeated cards), each player's hand is in no specific order, and in each hand there is a clear winner.</p>
// <p>How many hands does Player 1 win?</p>

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy)]
struct Card {
    pub value: char,
    pub suit: char,
}
impl Card {
    pub fn new(value: char, suit: char) -> Self {
        Card {
            value: value,
            suit: suit
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}
type CardSet = Vec<Card>;

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let valself = get_value_of_card(self);
        let valother = get_value_of_card(other);
        if valself < valother {
            Ordering::Less
        }
        else if valself == valother {
            Ordering::Equal
        }
        else {
            Ordering::Greater
        }
    }
}
impl Eq for Card { }
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

macro_rules! to_card {
    ($x:expr) => (Card::new($x.chars().nth(0).unwrap(), $x.chars().nth(1).unwrap()));
}

//const SUITS: Vec<char> = vec!['H', 'C', 'S', 'D'];
//const VALUES: Vec<char> = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const SUITS: &str = "HCSD";
const VALUES: &str = "23456789TJQKA";

fn main() {
    const INPUT: &str = "0054_poker.txt";

    let reader = BufReader::new(File::open(INPUT)
        .expect("Cannot open file.txt"));

    let mut games: Vec<(CardSet, CardSet)> = vec![];

    for line in reader.lines() {
        let (player1, player2) = split_line_to_player_cards(line.expect("Error reading line"));
        games.push((player1, player2));
    }

    for g in games.iter() {
        for c in &g.0 {
            print!("{} ", c);
        }
        print!(" -  ");
        for c in &g.1 {
            print!("{} ", c);
        }
        println!("");
    }

    let mut win_player1 = 0;

    for game in games {
        if check_player1_win(game.0, game.1) {
            win_player1 += 1;
        }
    }

    println!("Player 1 wins: {}", win_player1)
}

fn split_line_to_player_cards(line: String) -> (CardSet, CardSet) {
    let player1: CardSet = line
        .split_whitespace()
        .take(5)
        .map(|x| {Card::new(x.chars().nth(0).unwrap(), x.chars().nth(1).unwrap())} )
        .collect();
    let mut player2: CardSet = line
        .split_whitespace()
        .rev()
        .take(5)
        .map(|x| {Card::new(x.chars().nth(0).unwrap(), x.chars().nth(1).unwrap())} )
        .collect();
    player2.reverse();

    (player1, player2)
}

fn check_player1_win(cards_player1: CardSet, cards_player2: CardSet) -> bool {
    // rules (highest to lowest)
    //  1. straight flush, highest card in flush (incl royal flush)
    //  2. 4 of a kind, value of the four, value of other card
    //  3. 3 of a kind + 2 of a kind (full house), value of 3, value of 2
    //  4. flush (all same suit)
    //  5. straight (all consecutive values)
    //  6. 3 of a kind, value for the 3, highest values of other cards
    //  7. two pairs, value of pairs, value of other card
    //  8. one pair, value of pair, values of other cards
    //  9. highest card

    player_win_lose_royal_flush(&cards_player1, &cards_player2)
        .or_else(|| {player_win_lose_straight_flush(&cards_player1, &cards_player2)} )
        .or_else(|| {player_win_lose_four_of_a_kind(&cards_player1, &cards_player2)} )
        .or_else(|| {player_win_lose_full_house(&cards_player1, &cards_player2)} )
        .or_else(|| {player_win_lose_flush(&cards_player1, &cards_player2)} )
        .or_else(|| {player_win_lose_straight(&cards_player1, &cards_player2)} )
        .or_else(|| {player_win_lose_three_of_a_kind(&cards_player1, &cards_player2)} )
        .unwrap_or(false)
}


// Checks for ranked hands

fn player_win_lose_royal_flush(cards_player1: &CardSet, cards_player2: &CardSet) -> Option<bool> {
    //  1.1. royal flush
    let rf1 = is_royal_flush(&cards_player1);
    let rf2 = is_royal_flush(&cards_player2);

    if rf1 && !rf2 {
        Some(true)
    }
    else if rf2 {
        Some(false)
    }
    else {
        None
    }
}

fn player_win_lose_straight_flush(cards_player1: &CardSet, cards_player2: &CardSet) -> Option<bool> {
    //  1.2. straight flush, highest card in flush
    let sf1 = is_straight_flush(&cards_player1);
    let sf2 = is_straight_flush(&cards_player2);

    if sf1 && !sf2 {
        Some(true)
    }
    else if !sf1 && sf2 {
        Some(false)
    }
    else if sf1 && sf2 {
        Some(find_highest_value_card(&cards_player1).1 > find_highest_value_card(&cards_player2).1)
    }
    else {
        None
    }
}

fn player_win_lose_four_of_a_kind(cards_player1: &CardSet, cards_player2: &CardSet) -> Option<bool> {
    //  2. four of a kind, value of the four, value of other card
    let fok1 = find_n_of_a_kind(4, &cards_player1);
    let fok2 = find_n_of_a_kind(4, &cards_player2);

    if fok1.is_some() && fok2.is_none() {
        Some(true)
    }
    else if fok1.is_none() && fok2.is_some() {
        Some(false)
    }
    else if fok1.is_some() && fok2.is_some() {
        let hk1 = find_highest_value_card(&fok1.clone().unwrap().0).1;
        let hk2 = find_highest_value_card(&fok2.clone().unwrap().0).1;

        if hk1 == hk2 {
            let rest_hk1 = find_highest_value_card(&fok1.unwrap().1).1;
            let rest_hk2 = find_highest_value_card(&fok2.unwrap().1).1;
            Some(rest_hk1 > rest_hk2)
        }
        else {
            Some(hk1 > hk2)
        }
    }
    else {
        None
    }
}

fn player_win_lose_full_house(cards_player1: &CardSet, cards_player2: &CardSet) -> Option<bool> {
    //  3. 3 of a kind + 2 of a kind (full house), value of 3, value of 2
    let fh1 = get_full_house(&cards_player1);
    let fh2 = get_full_house(&cards_player2);

    if fh1.is_some() && fh2.is_none() {
        Some(true)
    }
    else if fh1.is_none() && fh2.is_some() {
        Some(false)
    }
    else if fh1.is_some() && fh2.is_some() {
        let fh1u = fh1.unwrap();
        let fh2u = fh2.unwrap();
        let (hk13, hk12) = (get_value_of_card(fh1u.0.iter().next().unwrap()), get_value_of_card(fh1u.1.iter().next().unwrap()));
        let (hk23, hk22) = (get_value_of_card(fh2u.0.iter().next().unwrap()), get_value_of_card(fh2u.1.iter().next().unwrap()));

        if hk13 > hk23 {
            Some(true)
        }
        else if (hk13 == hk23) && (hk12 > hk22) {
            Some(true)
        }
        else {
            Some(false)
        }
    }
    else {
        None
    }
}

fn player_win_lose_flush(cards_player1: &CardSet, cards_player2: &CardSet) -> Option<bool> {
    //  4. flush (all same suit)
    let flu1 = get_flush(&cards_player1);
    let flu2 = get_flush(&cards_player2);

    if flu1.is_some() && flu2.is_none() {
        Some(true)
    }
    else if flu1.is_none() && flu2.is_some() {
        Some(false)
    }
    else if flu1.is_some() && flu2.is_some() {
        for i in (0..5).rev() {
            let c1 = flu1.as_ref().unwrap().iter().nth(i).unwrap();
            let c2 = flu2.as_ref().unwrap().iter().nth(i).unwrap();
            if get_value_of_card(c1) < get_value_of_card(c2) {
                return Some(false)
            }
            else if get_value_of_card(c1) > get_value_of_card(c2) {
                return Some(true)
            }
        }
        Some(false)
    }
    else {
        None
    }
}

fn player_win_lose_straight(cards_player1: &CardSet, cards_player2: &CardSet) -> Option<bool> {
    //  5. straight (all consecutive values)
    let str1 = is_straight(cards_player1);
    let str2 = is_straight(cards_player2);

    if str1 && !str2 {
        println!("str1 here {:?} {:?}", cards_player1, cards_player2);
        Some(true)
    }
    else if !str1 && str2 {
        println!("str2 here {:?} {:?}", cards_player1, cards_player2);
        Some(false)
    }
    else if str1 && str2 {
        println!("str1 str2 here {:?} {:?}", cards_player1, cards_player2);
        let hkv1 = find_highest_value_card(cards_player1).1;
        let hkv2 = find_highest_value_card(cards_player2).1;

        Some(hkv1 > hkv2)
    }
    else {
        None
    }
}

fn player_win_lose_three_of_a_kind(cards_player1: &CardSet, cards_player2: &CardSet) -> Option<bool> {
    //  6. 3 of a kind, value for the 3, highest values of other cards
    let tok1 = find_n_of_a_kind(3, &cards_player1);
    let tok2 = find_n_of_a_kind(3, &cards_player2);

    println!("here {:?} {:?}", cards_player1, cards_player2);

    if tok1.is_some() && tok2.is_none() {
        println!("tok1 {:?}", tok1.clone().unwrap().0);
        Some(true)
    }
    else if tok1.is_none() && tok2.is_some() {
        println!("tok2 {:?}", tok2.clone().unwrap().0);
        Some(false)
    }
    else if tok1.is_some() && tok2.is_some() {
        let hk1 = find_highest_value_card(&tok1.clone().unwrap().0).1;
        let hk2 = find_highest_value_card(&tok2.clone().unwrap().0).1;
        println!("toksome1 {:?}, hk {}", tok1.clone().unwrap().0, hk1);
        println!("toksome2 {:?}, hk {}", tok2.clone().unwrap().0, hk2);

        if hk1 != hk2 {
            Some(hk1 > hk2)
        }
        else {
            let rest1 = &mut tok1.unwrap().1.clone();
            let rest2 = &mut tok2.unwrap().1.clone();
            rest1.sort();
            rest2.sort();
            println!("rest1 {:?}", rest1);
            println!("rest2 {:?}", rest2);

            for i in (0..2).rev() {
                let c1 = rest1.iter().nth(i).unwrap();
                let c2 = rest2.iter().nth(i).unwrap();
                if get_value_of_card(c1) < get_value_of_card(c2) {
                    return Some(false)
                }
                else if get_value_of_card(c1) > get_value_of_card(c2) {
                    return Some(true)
                }
            }
            Some(false)
        }
    }
    else {
        None
    }
}


// Helper functions

fn is_royal_flush(cards: &CardSet) -> bool {
    return is_straight(&cards) 
        && find_5_same_suit(&cards).is_some() 
        && 'A' == find_highest_value_card(&cards).0.value;
}

fn is_straight_flush(cards: &CardSet) -> bool {
    return is_straight(&cards) && find_5_same_suit(&cards).is_some();
}

fn is_straight(cards: &CardSet) -> bool {
    let mut values: Vec<usize> = cards
        .iter()
        .fold("".to_string(), |mut acc, b| {acc.push_str(&b.value.to_string()); acc})
        .chars()
        .map(|c| VALUES.chars().position(|x| x == c).unwrap())
        .collect();
    values.sort();

    let values_plus1: Vec<usize> = values.iter().map(|x| x+1).collect();

    values[1..].iter().zip(&values_plus1[..5]).all(|x| x.0 == x.1)
}

fn find_5_same_suit(cards: &CardSet) -> Option<CardSet> {
    for suit in SUITS.chars() {
        let suit_cards: CardSet = cards
            .iter()
            .filter(|c| c.suit == suit)
            .cloned()
            .collect();
        if suit_cards.len() == 5 {
            return Some(suit_cards);
        }
    }

    None
}

fn find_highest_value_card(cards: &CardSet) -> (Card, usize) {
    let mut highest_card: &Card = &cards[0];
    let mut highest_value: usize = 0;

    for card in cards {
        let val = get_value_of_card(card);
        if val > highest_value {
            highest_card = card;
            highest_value = val;
        }
    }

    (highest_card.clone(), highest_value)
}

fn get_value_of_card(card: &Card) -> usize {
    return VALUES.chars().position(|x| x == card.value).unwrap();
}

fn find_n_of_a_kind(n: usize, cards: &CardSet) -> Option<(CardSet, CardSet)> {
    let init_found_and_rest: (CardSet, CardSet) = (CardSet::new(), CardSet::new());

    for val in VALUES.chars() {
        let cards_by_val = cards.iter()
            .fold(
                init_found_and_rest.clone(),
                |mut acc, card| {if val == card.value {acc.0.push(card.clone())} else {acc.1.push(card.clone())} acc}
            );
            
        if cards_by_val.0.len() >= n {
            return Some(cards_by_val);
        }
    }

    None
}

fn get_full_house(cards: &CardSet) -> Option<(CardSet, CardSet)> {
    let three_of_a_kind = find_n_of_a_kind(3, cards);
    if three_of_a_kind.is_some() {
        let pair = find_n_of_a_kind(2, &three_of_a_kind.clone().unwrap().1);
        if pair.is_some() {
            return three_of_a_kind;
        }
    }

    None
}

fn get_flush(cards: &CardSet) -> Option<CardSet> {
    let same_suit = find_5_same_suit(cards);

    if same_suit.is_some() {
        let mut flush = same_suit.unwrap();
        flush.sort();
        return Some(flush);
    }

    None
}

#[cfg(test)]
macro_rules! vec_of_cards {
    ($($x:expr),*) => (vec![$(Card::new($x.chars().nth(0).unwrap(), $x.chars().nth(1).unwrap())),*]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_line_to_player_cards() {
        assert_eq!(split_line_to_player_cards("8C TS KC 9H 4S 7D 2S 5D 3S AC".to_string()), 
            //(vec_of_cards!["8C", "TS", "KC", "9H", "4S"],
            (vec_of_cards!["8C", "TS", "KC", "9H", "4S"],
             vec_of_cards!["7D", "2S", "5D", "3S", "AC"]));
    }

    #[test]
    fn test_check_player1_win() {
        assert_eq!(check_player1_win(vec_of_cards!["8C", "TS", "KC", "9H", "4S"], vec_of_cards!["8S", "TC", "KS", "9S", "AC"]), false);

        // single checks
        assert_eq!(player_win_lose_royal_flush(&vec_of_cards!["8C", "5H", "8S", "8D", "7C"], &vec_of_cards!["8D", "7S", "8C", "8H", "4S"]), None);
        assert_eq!(player_win_lose_straight_flush(&vec_of_cards!["8C", "5H", "8S", "8D", "7C"], &vec_of_cards!["8D", "7S", "8C", "8H", "4S"]), None);
        assert_eq!(player_win_lose_four_of_a_kind(&vec_of_cards!["8C", "5H", "8S", "8D", "7C"], &vec_of_cards!["8D", "7S", "8C", "8H", "4S"]), None);
        assert_eq!(player_win_lose_full_house(&vec_of_cards!["8C", "5H", "8S", "8D", "7C"], &vec_of_cards!["8D", "7S", "8C", "8H", "4S"]), None);
        assert_eq!(player_win_lose_flush(&vec_of_cards!["8C", "5H", "8S", "8D", "7C"], &vec_of_cards!["8D", "7S", "8C", "8H", "4S"]), None);
        assert_eq!(player_win_lose_straight(&vec_of_cards!["8C", "5H", "8S", "8D", "7C"], &vec_of_cards!["8D", "7S", "8C", "8H", "4S"]), None);

        // Royal Flush
        assert_eq!(check_player1_win(vec_of_cards!["KC", "TC", "JC", "QC", "AC"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        // Straight Flush
        assert_eq!(check_player1_win(vec_of_cards!["3C", "4C", "5C", "6C", "7C"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        // Four of a Kind
        assert_eq!(check_player1_win(vec_of_cards!["3C", "3H", "3S", "3D", "7C"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_cards!["3C", "3H", "3S", "3D", "7C"], vec_of_cards!["8D", "8S", "8C", "8H", "4S"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["AC", "AH", "AS", "AD", "7C"], vec_of_cards!["8D", "8S", "8C", "8H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_cards!["8C", "8H", "8S", "8D", "7C"], vec_of_cards!["8D", "8S", "8C", "8H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_cards!["8C", "8H", "8S", "8D", "4C"], vec_of_cards!["8D", "8S", "8C", "8H", "4S"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["8C", "8H", "8S", "8D", "2C"], vec_of_cards!["8D", "8S", "8C", "8H", "4S"]), false);
        // Full House
        assert_eq!(check_player1_win(vec_of_cards!["3C", "3H", "3S", "7D", "7C"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_cards!["3C", "3H", "3S", "7D", "7C"], vec_of_cards!["4C", "4S", "4D", "9H", "9S"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["8C", "TS", "KC", "9H", "4S"], vec_of_cards!["3C", "3H", "3S", "7D", "7C"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["3C", "3H", "3S", "7D", "7C"], vec_of_cards!["3C", "3H", "3S", "7D", "7C"]), false);
        // Flush
        assert_eq!(check_player1_win(vec_of_cards!["2C", "3C", "5C", "7C", "AC"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_cards!["2C", "3C", "5C", "7C", "TC"], vec_of_cards!["2D", "3D", "AD", "7D", "TD"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["2C", "3C", "5C", "7C", "TC"], vec_of_cards!["3D", "5D", "7D", "TD", "2D"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["2C", "3C", "5C", "7C", "TC"], vec_of_cards!["2D", "3D", "4D", "9D", "TD"]), false);
        // Straight
        assert_eq!(check_player1_win(vec_of_cards!["2C", "3D", "5C", "6H", "4C"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_cards!["2C", "3D", "5C", "6H", "4C"], vec_of_cards!["5C", "6S", "7C", "8H", "9S"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["2C", "3D", "5C", "6H", "5C"], vec_of_cards!["8C", "8H", "8S", "8D", "7C"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["6H", "7H", "TH", "9D", "8S"], vec_of_cards!["2C", "3D", "4H", "6H", "5C"]), true);
        // Three of a Kind

        assert_eq!(check_player1_win(vec_of_cards!["3C", "4H", "3S", "3D", "7C"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_cards!["3C", "4H", "3S", "3D", "7C"], vec_of_cards!["8D", "2S", "8C", "8H", "4S"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["2C", "AH", "AS", "AD", "7C"], vec_of_cards!["8D", "2S", "8C", "8H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_cards!["8C", "5H", "8S", "8D", "7C"], vec_of_cards!["8D", "7S", "8C", "8H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_cards!["8C", "2H", "8S", "8D", "4C"], vec_of_cards!["8D", "8S", "2C", "8H", "4S"]), false);
        assert_eq!(check_player1_win(vec_of_cards!["9C", "8H", "8S", "8D", "2C"], vec_of_cards!["8D", "AS", "8C", "8H", "4S"]), false);
        // assert_eq!(check_player1_win(vec_of_cards!["QC", "QH", "5S", "QD", "7C"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        // // Two Pairs
        // assert_eq!(check_player1_win(vec_of_cards!["QC", "QH", "5S", "5D", "7C"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        // // One Pair
        // assert_eq!(check_player1_win(vec_of_cards!["QC", "QH", "5S", "AD", "7C"], vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), true);
        // // High Card
        // assert_eq!(check_player1_win(vec_of_cards!["QC", "3H", "5S", "2D", "7C"], vec_of_cards!["8C", "TS", "4C", "9H", "2S"]), true);
    }

    #[test]
    fn test_is_royal_flush() {
        assert_eq!(is_royal_flush(&vec_of_cards!["8C", "TS", "KC", "9H", "4S"]), false);
        assert_eq!(is_royal_flush(&vec_of_cards!["KC", "TC", "JC", "QC", "AC"]), true);
        assert_eq!(is_royal_flush(&vec_of_cards!["7C", "6C", "5C", "4C", "8C"]), false);
    }

    #[test]
    fn test_is_straight_flush() {
        assert_eq!(is_straight_flush(&vec_of_cards!["7C", "6C", "5C", "4C", "8C"]), true);
        assert_eq!(is_straight_flush(&vec_of_cards!["7D", "6C", "5C", "4C", "8C"]), false);
    }

    #[test]
    fn test_is_straight() {
        assert_eq!(is_straight(&vec_of_cards!["KC", "TC", "JC", "QC", "AC"]), true);
        assert_eq!(is_straight(&vec_of_cards!["2C", "3C", "4C", "5C", "6C"]), true);
        assert_eq!(is_straight(&vec_of_cards!["7C", "6C", "5C", "4C", "8C"]), true);
        assert_eq!(is_straight(&vec_of_cards!["7C", "6C", "2C", "4C", "8C"]), false);
        assert_eq!(is_straight(&vec_of_cards!["6H", "7H", "TH", "9D", "8S"]), true);
        assert_eq!(is_straight(&vec_of_cards!["3H", "4H", "4H", "4D", "7S"]), false);
    }

    #[test]
    fn test_find_5_same_suit() {
        assert!(find_5_same_suit(&vec_of_cards!["8C", "TS", "KC", "9H", "4S"]).is_none());
        assert!(find_5_same_suit(&vec_of_cards!["8C", "TC", "KC", "9C", "4C"]).is_some());
        assert!(find_5_same_suit(&vec_of_cards!["7D", "6C", "5C", "4C", "8C"]).is_none());
    }

    #[test]
    fn test_find_highest_value_card() {
        assert_eq!(find_highest_value_card(&vec_of_cards!["8C", "TC", "KC", "9C", "4C"]), (to_card!["KC"], get_value_of_card(&to_card!["KC"])));
        assert_eq!(find_highest_value_card(&vec_of_cards!["8C", "AS", "KC", "9C", "4C"]), (to_card!["AS"], get_value_of_card(&to_card!["AS"])));
        assert_eq!(find_highest_value_card(&vec_of_cards!["2C", "3S", "2S", "3C", "4C"]), (to_card!["4C"], get_value_of_card(&to_card!["4C"])));
    }

    #[test]
    fn test_get_value_of_card() {
        assert!(get_value_of_card(&to_card!["8C"]) > get_value_of_card(&to_card!["2S"]));
        assert!(get_value_of_card(&to_card!["AD"]) > get_value_of_card(&to_card!["2S"]));
        assert!(get_value_of_card(&to_card!["KD"]) == get_value_of_card(&to_card!["KS"]));
    }

    #[test]
    fn test_find_n_of_a_kind() {
        assert_eq!(
            find_n_of_a_kind(4, &vec_of_cards!["8C", "8S", "8D", "9C", "8H"]), 
            Some((vec_of_cards!["8C", "8S", "8D", "8H"], vec_of_cards!["9C"]))
        );
        assert_eq!(
            find_n_of_a_kind(2, &vec_of_cards!["8C", "8S", "8D", "9C", "8H"]), 
            Some((vec_of_cards!["8C", "8S", "8D", "8H"], vec_of_cards!["9C"]))
        );
        assert_eq!(
            find_n_of_a_kind(4, &vec_of_cards!["8C", "2S", "8D", "9C", "8H"]), 
            None
        );
    }

    #[test]
    fn test_get_full_house() {
        assert_eq!(
            get_full_house(&vec_of_cards!["8C", "8S", "8D", "9C", "9H"]), 
            Some((vec_of_cards!["8C", "8S", "8D"], vec_of_cards!["9C", "9H"]))
        );
        assert_eq!(
            get_full_house(&vec_of_cards!["8C", "7S", "8D", "7C", "8H"]), 
            Some((vec_of_cards!["8C", "8D", "8H"], vec_of_cards!["7S", "7C"]))
        );
        assert_eq!(
            get_full_house(&vec_of_cards!["8C", "7S", "8D", "9C", "9H"]), 
            None
        );
        assert_eq!(
            get_full_house(&vec_of_cards!["8C", "7S", "8D", "9C", "8H"]), 
            None
        );
    }
}

// 
