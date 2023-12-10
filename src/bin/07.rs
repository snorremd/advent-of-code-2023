use itertools::Itertools;

advent_of_code::solution!(7);

fn char_to_card() -> impl Fn(char) -> usize {
    |card| match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as usize,
    }
}

fn input_to_hands(input: &str) -> impl Iterator<Item = (Vec<usize>, usize)> + '_ {
    input.lines().map(|line| {
        // Split lines on space into a hand and a bid
        let [hand_str, bid_str]: [&str; 2] =
            line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
        let hand = hand_str
            .chars()
            .map(char_to_card())
            .collect_vec();

        (hand, bid_str.parse::<usize>().unwrap())
    })
}

fn hand_type(hand: &Vec<usize>) -> usize {
    match hand
        .iter()
        .counts()
        .values()
        .sorted()
        .rev()
        .collect::<Vec<&usize>>()
        .as_slice()
    {
        [5] => 7,             // Five of a kind
        [4, 1] => 6,          // Four of a kind
        [3, 2] => 5,          // Full house
        [3, 1, 1] => 4,       // Three of a kind
        [2, 2, 1] => 3,       // Two pair
        [2, 1, 1, 1] => 2,    // One pair
        [1, 1, 1, 1, 1] => 1, // High card
        _ => panic!("Invalid hand"),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let hands = input_to_hands(input);

    let score = hands
        .map(|(hand, bid)| {
            let hand_type: usize = hand_type(&hand);
            (hand, hand_type, bid)
        })
        .sorted_by(|(ha, hta, ..), (hb, htb, ..)| {
            hta.cmp(htb).then(ha.cmp(hb))
        })
        .enumerate()
        .fold(0, |acc, (index, (_, _, bid))| {
            // Calculate score
            acc + (bid * (index + 1))
        });

    Some(score)
}



pub fn part_two(input: &str) -> Option<usize> {
    let hands = input_to_hands(input);
    let cards = 2_usize..=14;
    // For each hand replace jokers (11) with all possible cards
    // E.g. [11, 11, 11, 11, 11] -> [[2, 11, 11, 11, 11], [3, 11, 11, 11, 11], ...]
    // If there is more than one joker, the number of hands will be 13^jokers
    let new_cards = hands.map(|(hand, bid)| {
        let positions = hand.iter().positions(|card| { *card == 11 }).collect_vec();

        let combinations = cards.clone().combinations_with_replacement(positions.len());
        combinations.map(move |combination| {
            let mut typehand = hand.clone();
            let mut sorthand = hand.clone();
            for (index, card) in combination.iter().enumerate() {
                typehand[positions[index]] = card.clone();
                sorthand[positions[index]] = 1;
            }
            let hand_type = hand_type(&typehand);
            (typehand, sorthand, hand_type, bid)
        }).sorted_by(|(_, ha, hta, ..), (_, hb, htb, ..)| {
            htb.cmp(hta).then(hb.cmp(ha))
        }).take(1)

    })
    .flatten().collect_vec();

    let score = new_cards.iter()
    .sorted_by(|(_, ha, hta, ..), (_, hb, htb, ..)| {
        hta.cmp(htb).then(ha.cmp(hb))
    })
    .enumerate()
    .fold(0, |acc, (index, (_, _, _, bid))| {
        // Calculate score
        acc + (bid * (index + 1))
    });

    Some(score)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
