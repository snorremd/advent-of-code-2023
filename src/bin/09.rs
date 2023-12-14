advent_of_code::solution!(9);

fn extrapolate(history: &Vec<i32>) -> i32 {
    let mut diffs = Vec::with_capacity(history.len() - 1);
    for pair in history.windows(2) {
        diffs.push(pair[1] - pair[0]);
    }

    if diffs.iter().all(|&diff| diff == 0) {
        history.last().unwrap() + 0
    } else {
        history.last().unwrap() + extrapolate(&diffs)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let score = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|history| extrapolate(&history))
        .sum();

    Some(score)
}

pub fn part_two(input: &str) -> Option<i32> {
    let score = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|mut history| {
            history.reverse();
            extrapolate(&history)
        })
        .sum();

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
