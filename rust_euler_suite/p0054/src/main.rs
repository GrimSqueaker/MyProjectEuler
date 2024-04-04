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

type CardSet = Vec<String>;

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
        .map(String::from)
        .collect();
    let mut player2: CardSet = line
        .split_whitespace()
        .rev()
        .take(5)
        .map(String::from)
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
    //  6. 3 of a kind, value fo the 3, values of other cards
    //  7. two pairs, value of pairs, value of other card
    //  8. one pair, value of pair, values of other cards
    //  9. highest card

    //  1. straight flush, highest card in flush (incl royal flush)
    let rf1 = is_royal_flush(&cards_player1);
    let rf2 = is_royal_flush(&cards_player2);

    if rf1 && !rf2 {
        return true;
    }
    else if rf2 {
        return false;
    }

    let sf1 = is_straight_flush(&cards_player1);
    let sf2 = is_straight_flush(&cards_player2);

    if sf1 && !sf2 {
        return true;
    }
    else if !sf1 && sf2 {
        return false;
    }
    else if sf1 && sf2 {
        return find_highest_value_card(&cards_player1).1 > find_highest_value_card(&cards_player2).1;
    }

    //  2. four of a kind, value of the four, value of other card
    let fok1 = find_n_of_a_kind(4, &cards_player1);
    let fok2 = find_n_of_a_kind(4, &cards_player2);

    if fok1.is_some() && fok2.is_none() {
        return true;
    }
    else if fok1.is_none() && fok2.is_some() {
        return false;
    }
    else if fok1.is_some() && fok2.is_some() {
        let hk1 = find_highest_value_card(&fok1.clone().unwrap().0).1;
        let hk2 = find_highest_value_card(&fok2.clone().unwrap().0).1;
        if hk1 == hk2 {
            let rest_hk1 = find_highest_value_card(&fok1.unwrap().1).1;
            let rest_hk2 = find_highest_value_card(&fok2.unwrap().1).1;
            return rest_hk1 > rest_hk2;
        }
        else {
            return hk1 > hk2;
        }
    }

    false
}

fn is_royal_flush(cards: &CardSet) -> bool {
    return is_straight(&cards) 
        && find_5_same_suit(&cards).is_some() 
        && 'A' == find_highest_value_card(&cards).0.chars().next().unwrap();
}

fn is_straight_flush(cards: &CardSet) -> bool {
    return is_straight(&cards) && find_5_same_suit(&cards).is_some();
}

fn is_straight(cards: &CardSet) -> bool {
    let mut values: Vec<char> = cards
        .iter()
        .fold("".to_string(), |mut acc, b| {acc.push_str(&b.chars().nth(0).unwrap().to_string()); acc})
        .chars()
        .collect();
    values.sort_by_key(|k| VALUES.chars().position(|c| c == *k).unwrap());
    let values_string: String = values.into_iter().collect();

    let val_first = VALUES.chars().position(|x| x == values_string.chars().next().unwrap()).unwrap();
    let val_last = VALUES.chars().position(|x| x == values_string.chars().last().unwrap()).unwrap();

    return (val_last-val_first) == cards.len()-1;
}

fn find_5_same_suit(cards: &CardSet) -> Option<CardSet> {
    for suit in SUITS.chars() {
        let suit_cards: CardSet = cards
            .iter()
            .filter(|c| c.chars().nth(1).unwrap() == suit)
            .cloned()
            .collect();
        if suit_cards.len() == 5 {
            return Some(suit_cards);
        }
    }

    None
}

fn find_highest_value_card(cards: &CardSet) -> (String, usize) {
    let mut highest_card: &String = &cards[0];
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

fn get_value_of_card(card: &str) -> usize {
    let value = card.chars().next().unwrap();
    return VALUES.chars().position(|x| x == value).unwrap();
}

fn find_n_of_a_kind(n: usize, cards: &CardSet) -> Option<(CardSet, CardSet)> {
    let init_found_and_rest: (CardSet, CardSet) = (CardSet::new(), CardSet::new());

    for val in VALUES.chars() {
        let cards_by_val = cards.iter()
            .fold(
                init_found_and_rest.clone(),
                |mut acc, card| {if val == card.chars().next().unwrap() {acc.0.push(card.clone())} else {acc.1.push(card.clone())} acc}
            );
            
        if cards_by_val.0.len() >= n {
            return Some(cards_by_val);
        }
    }

    None
}

#[cfg(test)]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_line_to_player_cards() {
        assert_eq!(split_line_to_player_cards("8C TS KC 9H 4S 7D 2S 5D 3S AC".to_string()), 
            (vec_of_strings!["8C", "TS", "KC", "9H", "4S"],
             vec_of_strings!["7D", "2S", "5D", "3S", "AC"]));
    }

    #[test]
    fn test_check_player1_win() {
        assert_eq!(check_player1_win(vec_of_strings!["8C", "TS", "KC", "9H", "4S"], vec_of_strings!["8S", "TC", "KS", "9S", "AC"]), false);

        // trivial wins
        // Royal Flush
        assert_eq!(check_player1_win(vec_of_strings!["KC", "TC", "JC", "QC", "AC"], vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), true);
        // Straight Flush
        assert_eq!(check_player1_win(vec_of_strings!["3C", "4C", "5C", "6C", "7C"], vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), true);
        // // Four of a Kind
        assert_eq!(check_player1_win(vec_of_strings!["3C", "3H", "3S", "3D", "7C"], vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_strings!["3C", "3H", "3S", "3D", "7C"], vec_of_strings!["8D", "8S", "8C", "8H", "4S"]), false);
        assert_eq!(check_player1_win(vec_of_strings!["AC", "AH", "AS", "AD", "7C"], vec_of_strings!["8D", "8S", "8C", "8H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_strings!["8C", "8H", "8S", "8D", "7C"], vec_of_strings!["8D", "8S", "8C", "8H", "4S"]), true);
        assert_eq!(check_player1_win(vec_of_strings!["8C", "8H", "8S", "8D", "4C"], vec_of_strings!["8D", "8S", "8C", "8H", "4S"]), false);
        assert_eq!(check_player1_win(vec_of_strings!["8C", "8H", "8S", "8D", "2C"], vec_of_strings!["8D", "8S", "8C", "8H", "4S"]), false);
        // // Full House
        // assert_eq!(check_player1_win(vec_of_strings!["3C", "3H", "3S", "7D", "7C"], vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), true);
        // // Flush
        // assert_eq!(check_player1_win(vec_of_strings!["2C", "3C", "5C", "7C", "AC"], vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), true);
        // // Straight
        // assert_eq!(check_player1_win(vec_of_strings!["3C", "4H", "5S", "6D", "7C"], vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), true);
        // // Three of a Kind
        // assert_eq!(check_player1_win(vec_of_strings!["QC", "QH", "5S", "QD", "7C"], vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), true);
        // // Two Pairs
        // assert_eq!(check_player1_win(vec_of_strings!["QC", "QH", "5S", "5D", "7C"], vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), true);
        // // One Pair
        // assert_eq!(check_player1_win(vec_of_strings!["QC", "QH", "5S", "AD", "7C"], vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), true);
        // // High Card
        // assert_eq!(check_player1_win(vec_of_strings!["QC", "3H", "5S", "2D", "7C"], vec_of_strings!["8C", "TS", "4C", "9H", "2S"]), true);

        // // wins
        // assert_eq!(check_player1_win(vec_of_strings!["QC", "QH", "5S", "AD", "7C"], vec_of_strings!["QC", "4S", "KC", "9H", "QS"]), true);

        // loses
    }

    #[test]
    fn test_is_royal_flush() {
        assert_eq!(is_royal_flush(&vec_of_strings!["8C", "TS", "KC", "9H", "4S"]), false);
        assert_eq!(is_royal_flush(&vec_of_strings!["KC", "TC", "JC", "QC", "AC"]), true);
        assert_eq!(is_royal_flush(&vec_of_strings!["7C", "6C", "5C", "4C", "8C"]), false);
    }

    #[test]
    fn test_is_straight_flush() {
        assert_eq!(is_straight_flush(&vec_of_strings!["7C", "6C", "5C", "4C", "8C"]), true);
        assert_eq!(is_straight_flush(&vec_of_strings!["7D", "6C", "5C", "4C", "8C"]), false);
    }

    #[test]
    fn test_is_straight() {
        assert_eq!(is_straight(&vec_of_strings!["KC", "TC", "JC", "QC", "AC"]), true);
        assert_eq!(is_straight(&vec_of_strings!["2C", "3C", "4C", "5C", "6C"]), true);
        assert_eq!(is_straight(&vec_of_strings!["7C", "6C", "5C", "4C", "8C"]), true);
        assert_eq!(is_straight(&vec_of_strings!["7C", "6C", "2C", "4C", "8C"]), false);
    }

    #[test]
    fn test_find_5_same_suit() {
        assert!(find_5_same_suit(&vec_of_strings!["8C", "TS", "KC", "9H", "4S"]).is_none());
        assert!(find_5_same_suit(&vec_of_strings!["8C", "TC", "KC", "9C", "4C"]).is_some());
        assert!(find_5_same_suit(&vec_of_strings!["7D", "6C", "5C", "4C", "8C"]).is_none());
    }

    #[test]
    fn test_find_highest_value_card() {
        assert_eq!(find_highest_value_card(&vec_of_strings!["8C", "TC", "KC", "9C", "4C"]), ("KC".to_string(), get_value_of_card("KC")));
        assert_eq!(find_highest_value_card(&vec_of_strings!["8C", "AS", "KC", "9C", "4C"]), ("AS".to_string(), get_value_of_card("AS")));
        assert_eq!(find_highest_value_card(&vec_of_strings!["2C", "3S", "2S", "3C", "4C"]), ("4C".to_string(), get_value_of_card("4C")));
    }

    #[test]
    fn test_get_value_of_card() {
        assert!(get_value_of_card("8C") > get_value_of_card("2S"));
        assert!(get_value_of_card("AD") > get_value_of_card("2S"));
        assert!(get_value_of_card("KD") == get_value_of_card("KS"));
    }

    #[test]
    fn test_find_n_of_a_kind() {
        assert_eq!(
            find_n_of_a_kind(4, &vec_of_strings!["8C", "8S", "8D", "9C", "8H"]), 
            Some((vec_of_strings!["8C", "8S", "8D", "8H"], vec_of_strings!["9C"]))
        );
        assert_eq!(
            find_n_of_a_kind(2, &vec_of_strings!["8C", "8S", "8D", "9C", "8H"]), 
            Some((vec_of_strings!["8C", "8S", "8D", "8H"], vec_of_strings!["9C"]))
        );
        assert_eq!(
            find_n_of_a_kind(4, &vec_of_strings!["8C", "2S", "8D", "9C", "8H"]), 
            None
        );
    }
}

// 
