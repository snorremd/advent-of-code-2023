advent_of_code::solution!(2);

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    number: usize,
    draws: Vec<Cubes>,
}


fn games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            // Split line on space or colon
            let number = line.split(|c| c == ' ' || c == ':').nth(1).unwrap().parse::<usize>().unwrap();
            let draws = line
                .split(":")
                .nth(1).unwrap()
                .split(";")
                .map(|draws| {
                    let mut cubes = Cubes {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };

                    draws.split(",").for_each(|draw| {
                        let mut draw = draw.trim().split(" ");
                        let number = draw.next().unwrap().parse::<u32>().unwrap();
                        let color = draw.next().unwrap();
                        match color {
                            "red" => cubes.red = number,
                            "green" => cubes.green = number,
                            "blue" => cubes.blue = number,
                            _ => panic!("Unknown color: {}", color),
                        }
                    });

                cubes
            }).collect::<Vec<Cubes>>();

            Game {
                number,
                draws,
            }
        }).collect::<Vec<Game>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let bag = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let score = games(input)
        .iter()
        .filter(|game| {
            game.draws.iter().all(|draw| {
                draw.red <= bag.red
                    && draw.green <= bag.green
                    && draw.blue <= bag.blue
            })
        })
        .map(|game| game.number)
        .sum::<usize>();

        Some(score as u32)

}

pub fn part_two(input: &str) -> Option<u32> {
    let score = games(input)
        .iter()
        .map(|Game{  draws, .. }| {
            let mut bag = Cubes {
                red: 0,
                green: 0,
                blue: 0,
            };
            draws.iter().for_each(|draw| {
                if draw.red > bag.red {
                    bag.red = draw.red;
                }

                if draw.green > bag.green {
                    bag.green = draw.green;
                }

                if draw.blue > bag.blue {
                    bag.blue = draw.blue;
                }
            });

            bag
        })
        .map(|Cubes{red, green, blue}| red * green * blue)
        .sum::<u32>();


    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
