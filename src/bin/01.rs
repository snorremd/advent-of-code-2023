advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = input.split_whitespace()
        // Replace all a-z with nothing using regex
        .map(|s| s.replace(|c: char| c.is_alphabetic(), ""))
        // Grab first and last characters
        .map(|s| s.chars().take(1).chain(s.chars().rev().take(1)).collect::<String>())
        .collect::<Vec<String>>();

    let res = numbers
        .iter()
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .sum::<u32>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits_as_string = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let res: u32 = input.split_whitespace()
        .map(|s| {
            // Iterate through string trying to find first digit or text representation of digit
            let nums = s.chars().enumerate().map(|(i, c)| {
                if c.is_digit(10) {
                    return c.to_digit(10).unwrap()
                }
                // Loop through digits_as_string and check if it matches
                for (j, d) in digits_as_string.iter().enumerate() {
                    if s[i..].starts_with(d) {
                        return (j + 1) as u32
                    }
                }
                0
            })
            .filter(|&n| n != 0)
            .collect::<Vec<u32>>();
            // Use string format to turn first and last number into two digit number
            format!("{}{}", nums[0], nums[nums.len() - 1])
                .parse::<u32>()
                .unwrap()

        }).sum::<u32>();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
