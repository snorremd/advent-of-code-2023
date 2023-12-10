use itertools::{multizip, Itertools};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    // Parse input into tuples of (time, speed) in usize based on
    // Time:      7  15   30
    // Distance:  9  40  200
    let mut times_distances = input
        .lines()
        .map(|s| s.split_whitespace().filter_map(|s| s.parse::<usize>().ok()));

    let times = times_distances.next().unwrap();
    let distances = times_distances.next().unwrap();
    let rounds = times.zip(distances);

    let score = rounds
        .map(|(time, record_distance)| {
            (0..time)
                .map(|press| {
                    let remaining = time - press;
                    let distance = press * remaining;
                    distance
                })
                .filter(|&distance| distance > record_distance)
                .count()
        })
        .reduce(|a, b| a * b);

    score
}

pub fn part_two(input: &str) -> Option<usize> {
    let [time, speed]: [usize; 2] = input
        .lines()
        .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<String>())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_vec()
        .try_into()
        .unwrap();

    (0..time)
        .map(|press| {
            let remaining = time - press;
            let distance = press * remaining;
            distance
        })
        .filter(|&distance| distance > speed)
        .count()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
