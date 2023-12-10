use itertools::{Itertools, izip};

advent_of_code::solution!(4);

fn line_to_tuple(line: &str) -> (Vec<u8>, Vec<u8>) {
    let (winning, have): (Vec<u8>, Vec<u8>) = line
        .split([':', '|'])
        .skip(1)
        .map(|s| s.split(' ').filter_map(|s| s.parse::<u8>().ok()).collect_vec())
        .map(|mut v| {
            v.sort_unstable();
            v
        }).take(2).collect_tuple().unwrap();

    (winning, have)
}

fn card_to_match_count((winning, have): (Vec<u8>, Vec<u8>)) -> usize {
    // Compare winning numbers (first vec) with the numbers we have by walking over both vecs simultaneously
    let mut winning = winning.iter();
    let mut have = have.iter();
    let mut matches: usize = 0;

    let mut curr_win = winning.next();
    let mut curr_have = have.next();

    while let (Some(w), Some(h)) = (curr_win, curr_have) {
        match (w, h) {
            (w, h) if w == h => {
                matches += 1;
                curr_win = winning.next();
                curr_have = have.next();
            },
            (w, h) if w > h => curr_have = have.next(),
            (w, h) if w < h => curr_win = winning.next(),
            _ => (),
        };
    }

    matches
}

pub fn part_one(input: &str) -> Option<usize> {
    let score = input
        .lines()
        .map(line_to_tuple)
        .map(card_to_match_count)
        .map(|score| {
                // Return 2 ^ score
            match score {
                0 => 0_usize,
                _ => 2_usize.pow((score - 1) as u32),
            }
        })
        .sum::<usize>();

    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut card_matches = izip!(0.., input
        .lines()
        .map(line_to_tuple)
        .map(card_to_match_count),
        std::iter::repeat::<usize>(1)).collect_vec();

    let mut i = 0;
    while i < card_matches.len() {
        match card_matches[i].2 {
            0 =>  {},
            to_add => {
                // We have matches here, so increase next n cards by count
                let card = card_matches[i].0;
                let n = card_matches[i].1;
                let to = (card + n).min(card_matches.len() - 1);
                for c in (card+1)..=to {
                    card_matches[c].2 += to_add;
                }
            }
        };
        i += 1;
    }

    let score = card_matches.iter().map(|(_, _, count)| count).sum::<usize>();

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
