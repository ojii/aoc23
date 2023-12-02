use crate::{parse_lines, FromLine};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Color {
    Blue,
    Red,
    Green,
}

impl Color {
    fn max(&self) -> usize {
        match self {
            Color::Blue => 14,
            Color::Red => 12,
            Color::Green => 13,
        }
    }
    fn parse(s: &str) -> Option<Self> {
        match s {
            "Blue" => Some(Self::Blue),
            "Red" => Some(Self::Red),
            "Green" => Some(Self::Green),
            _ => None,
        }
    }
}

#[derive(Default)]
struct Cubes {
    blue: Option<usize>,
    red: Option<usize>,
    green: Option<usize>,
}

impl Cubes {
    fn get(&self, color: &Color) -> Option<usize> {
        match color {
            Color::Blue => self.blue,
            Color::Red => self.red,
            Color::Green => self.green,
        }
    }
    fn with(self, color: &Color, num: usize) -> Self {
        match color {
            Color::Blue => Cubes {
                blue: Some(num),
                red: self.red,
                green: self.green,
            },
            Color::Red => Cubes {
                blue: self.blue,
                red: Some(num),
                green: self.green,
            },
            Color::Green => Cubes {
                blue: self.blue,
                red: self.red,
                green: Some(num),
            },
        }
    }
}

struct Game {
    id: usize,
    cubes: Vec<Cubes>,
}

impl Game {
    fn get<'a>(&'a self, color: &'a Color) -> impl Iterator<Item = usize> + 'a {
        self.cubes.iter().flat_map(|cubes| cubes.get(color))
    }
}

impl FromLine for Game {
    fn from_line(line: &str) -> Option<Self> {
        let (game_id, cube_sets) = line[5..].split_once(": ")?;
        let id = game_id.parse().ok()?;
        let cubes = cube_sets
            .split("; ")
            .map(|cubes| {
                cubes
                    .split(", ")
                    .flat_map(|cube| cube.split_once(' '))
                    .flat_map(|(num, color)| {
                        num.parse::<usize>()
                            .ok()
                            .and_then(|num| Color::parse(color).map(|color| (num, color)))
                    })
                    .fold(Cubes::default(), |cubes, (num, color)| {
                        cubes.with(&color, num)
                    })
            })
            .collect();
        Some(Self { id, cubes })
    }
}

fn part_1(games: &[Game]) -> usize {
    games
        .iter()
        .filter(|game| Color::iter().all(|color| game.get(&color).all(|num| num <= color.max())))
        .map(|game| game.id)
        .sum()
}

fn part_2(games: &[Game]) -> usize {
    games
        .iter()
        .map(|game| {
            Color::iter()
                .map(|color| game.get(&color).max().unwrap_or(0))
                .reduce(|a, b| a * b)
                .unwrap_or(0)
        })
        .sum()
}

pub fn day2() {
    let games: Vec<_> = parse_lines(INPUT).collect();
    println!("{}", part_1(&games));
    println!("{}", part_2(&games));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "Game 1: 3 Blue, 4 Red; 1 Red, 2 Green, 6 Blue; 2 Green
Game 2: 1 Blue, 2 Green; 3 Green, 4 Blue, 1 Red; 1 Green, 1 Blue
Game 3: 8 Green, 6 Blue, 20 Red; 5 Blue, 4 Red, 13 Green; 5 Green, 1 Red
Game 4: 1 Green, 3 Red, 6 Blue; 3 Green, 6 Red; 3 Green, 15 Blue, 14 Red
Game 5: 6 Red, 1 Blue, 3 Green; 2 Blue, 1 Red, 2 Green";
        assert_eq!(part_1(&parse_lines(data).collect::<Vec<_>>()), 8)
    }

    #[test]
    fn test_part_2() {
        let data = "Game 1: 3 Blue, 4 Red; 1 Red, 2 Green, 6 Blue; 2 Green
Game 2: 1 Blue, 2 Green; 3 Green, 4 Blue, 1 Red; 1 Green, 1 Blue
Game 3: 8 Green, 6 Blue, 20 Red; 5 Blue, 4 Red, 13 Green; 5 Green, 1 Red
Game 4: 1 Green, 3 Red, 6 Blue; 3 Green, 6 Red; 3 Green, 15 Blue, 14 Red
Game 5: 6 Red, 1 Blue, 3 Green; 2 Blue, 1 Red, 2 Green";
        assert_eq!(part_2(&parse_lines(data).collect::<Vec<_>>()), 2286)
    }
}

const INPUT: &str = "Game 1: 4 Green, 2 Blue; 1 Red, 1 Blue, 4 Green; 3 Green, 4 Blue, 1 Red; 7 Green, 2 Blue, 4 Red; 3 Red, 7 Green; 3 Red, 3 Green
Game 2: 1 Blue, 11 Red, 1 Green; 3 Blue, 2 Red, 4 Green; 11 Red, 2 Green, 2 Blue; 13 Green, 5 Red, 1 Blue; 4 Green, 8 Red, 3 Blue
Game 3: 9 Red, 2 Blue; 4 Blue, 2 Green, 1 Red; 7 Red, 4 Blue, 3 Green; 3 Blue, 6 Red; 9 Blue, 4 Red; 3 Red
Game 4: 5 Blue, 11 Green, 3 Red; 6 Green, 3 Blue, 7 Red; 17 Blue, 9 Green; 1 Red, 5 Blue, 3 Green; 6 Red, 7 Blue, 4 Green
Game 5: 3 Green, 7 Blue, 7 Red; 6 Green, 3 Red, 4 Blue; 7 Blue, 4 Red
Game 6: 1 Green, 3 Blue; 2 Blue, 9 Red; 2 Green, 13 Blue, 11 Red; 7 Red, 12 Blue, 1 Green
Game 7: 2 Blue, 6 Red, 12 Green; 7 Red, 8 Blue, 6 Green; 7 Blue, 3 Green, 7 Red; 5 Blue, 9 Green, 13 Red
Game 8: 13 Blue, 1 Green; 3 Red, 9 Blue; 3 Red, 4 Blue; 2 Red, 3 Blue, 1 Green; 1 Green, 15 Blue, 4 Red
Game 9: 1 Green, 5 Blue, 11 Red; 2 Red, 1 Blue; 2 Red, 5 Blue
Game 10: 8 Red, 20 Green; 12 Green, 1 Red, 2 Blue; 5 Red, 3 Blue, 7 Green; 4 Red, 19 Green, 6 Blue; 3 Blue, 4 Red, 14 Green; 9 Red, 15 Green
Game 11: 7 Green, 4 Blue, 14 Red; 7 Red, 8 Green; 6 Blue, 6 Red; 5 Blue, 10 Red, 11 Green; 12 Red, 2 Green
Game 12: 4 Blue, 5 Green, 8 Red; 2 Green, 4 Blue, 7 Red; 4 Blue, 3 Green, 2 Red; 2 Red, 4 Green
Game 13: 7 Blue, 8 Red; 5 Green, 15 Blue, 2 Red; 7 Green, 3 Blue, 12 Red
Game 14: 4 Green, 16 Red; 6 Red, 2 Green; 5 Red, 1 Blue, 3 Green; 1 Blue, 1 Red, 2 Green
Game 15: 3 Green; 2 Blue, 1 Red, 2 Green; 6 Blue; 3 Blue, 1 Red, 2 Green; 2 Red, 1 Green
Game 16: 13 Green, 3 Red; 9 Green, 1 Blue; 4 Blue, 1 Red, 18 Green; 2 Red, 3 Blue, 7 Green; 17 Green, 2 Red, 3 Blue; 12 Green, 2 Red
Game 17: 2 Blue, 4 Green, 3 Red; 2 Red, 5 Green, 11 Blue; 5 Green, 15 Blue, 2 Red; 3 Green, 13 Blue; 6 Blue, 2 Green, 2 Red; 8 Blue, 1 Red
Game 18: 6 Red, 4 Green, 7 Blue; 2 Red, 3 Green, 12 Blue; 3 Red, 6 Blue, 6 Green; 9 Red, 10 Blue; 6 Green, 4 Blue, 2 Red; 12 Red, 12 Blue, 9 Green
Game 19: 3 Blue, 2 Red, 3 Green; 16 Red, 3 Blue, 5 Green; 2 Red, 6 Green; 3 Green, 2 Blue, 15 Red; 2 Blue, 13 Red, 1 Green
Game 20: 2 Blue; 1 Green, 5 Blue, 2 Red; 3 Blue, 2 Red, 1 Green; 1 Red, 2 Blue
Game 21: 15 Green, 13 Blue, 4 Red; 9 Green, 6 Red, 19 Blue; 6 Blue, 1 Green, 1 Red; 1 Red, 11 Green, 9 Blue; 3 Red, 14 Green, 8 Blue
Game 22: 3 Blue, 10 Red, 1 Green; 2 Red, 6 Green; 9 Green, 3 Blue, 4 Red; 2 Blue, 4 Green
Game 23: 5 Red, 2 Green, 5 Blue; 4 Green, 12 Red, 2 Blue; 3 Green, 8 Red, 4 Blue
Game 24: 1 Green, 16 Red, 3 Blue; 10 Red, 1 Blue; 2 Blue, 1 Green, 7 Red; 12 Red, 1 Green; 14 Red, 1 Green; 1 Blue, 8 Red, 1 Green
Game 25: 8 Blue, 9 Red, 6 Green; 2 Blue, 4 Green, 8 Red; 1 Green, 9 Blue, 2 Red; 14 Red, 4 Blue
Game 26: 4 Blue, 3 Green; 1 Red, 3 Blue; 6 Red, 2 Green, 6 Blue; 5 Green, 2 Red; 5 Blue, 5 Green; 6 Red, 1 Blue
Game 27: 6 Green, 9 Blue; 1 Red, 6 Green, 8 Blue; 3 Green, 1 Blue, 1 Red; 3 Red, 4 Blue; 2 Red, 2 Blue; 4 Red, 3 Green, 7 Blue
Game 28: 5 Green, 2 Blue; 5 Blue; 1 Red, 4 Blue, 3 Green; 1 Green, 2 Red
Game 29: 1 Green, 2 Red, 4 Blue; 1 Green, 2 Red, 1 Blue; 9 Red, 6 Blue
Game 30: 1 Green, 1 Red, 5 Blue; 13 Blue, 4 Green, 2 Red; 10 Green, 11 Blue; 9 Green, 2 Red, 12 Blue
Game 31: 4 Red, 5 Blue; 8 Blue, 1 Red, 1 Green; 4 Red, 5 Green; 3 Green; 9 Blue, 2 Red, 7 Green
Game 32: 5 Blue, 4 Red, 5 Green; 10 Red, 10 Green, 5 Blue; 10 Red, 12 Green, 6 Blue; 8 Red, 1 Blue, 13 Green; 6 Green, 14 Red, 2 Blue
Game 33: 9 Green, 6 Red, 4 Blue; 1 Red, 2 Blue, 13 Green; 4 Red, 4 Green, 5 Blue
Game 34: 1 Blue, 1 Red; 9 Green, 14 Red, 1 Blue; 3 Blue, 7 Green
Game 35: 1 Red, 11 Green, 5 Blue; 1 Red, 5 Blue, 17 Green; 19 Green, 6 Blue; 4 Green, 7 Blue; 10 Blue, 7 Green
Game 36: 9 Green, 6 Blue, 4 Red; 8 Blue, 13 Green, 1 Red; 5 Blue, 5 Green; 15 Green, 1 Red
Game 37: 1 Green, 9 Red, 1 Blue; 14 Green; 11 Green, 6 Red
Game 38: 2 Blue; 9 Green, 1 Blue, 8 Red; 4 Green, 1 Blue, 3 Red
Game 39: 7 Red, 7 Blue; 3 Green, 6 Blue, 2 Red; 3 Green, 4 Red
Game 40: 5 Blue, 2 Red, 6 Green; 6 Blue, 10 Green, 4 Red; 8 Green, 6 Blue; 3 Green, 2 Blue; 2 Red, 14 Green
Game 41: 5 Red, 14 Blue, 3 Green; 3 Red, 3 Blue, 7 Green; 19 Blue, 15 Green, 6 Red; 5 Green, 18 Blue; 1 Green, 7 Red, 9 Blue; 14 Green, 10 Blue, 1 Red
Game 42: 2 Red, 3 Green; 2 Blue, 3 Red; 15 Green, 1 Blue; 2 Blue, 15 Green, 1 Red; 7 Red, 15 Green
Game 43: 4 Green, 6 Red, 9 Blue; 4 Green, 3 Red, 18 Blue; 6 Green, 7 Blue; 4 Red, 7 Blue; 8 Blue, 7 Green, 1 Red; 5 Red, 14 Blue
Game 44: 2 Green, 11 Blue; 1 Green, 5 Red, 8 Blue; 4 Green, 17 Blue, 4 Red
Game 45: 6 Blue, 3 Green, 2 Red; 8 Green, 12 Blue, 3 Red; 13 Blue, 11 Green; 13 Blue, 9 Green; 2 Blue, 3 Green, 3 Red; 2 Blue, 10 Green
Game 46: 14 Blue, 12 Green, 3 Red; 2 Green, 1 Red, 10 Blue; 5 Red, 7 Green
Game 47: 15 Blue, 1 Red; 1 Red, 14 Blue; 1 Red, 16 Blue; 3 Green, 8 Blue
Game 48: 1 Green, 3 Blue, 1 Red; 8 Blue, 2 Red, 8 Green; 14 Red, 4 Green, 11 Blue
Game 49: 6 Red, 5 Blue, 2 Green; 3 Red, 11 Blue; 1 Blue, 14 Green, 6 Red
Game 50: 7 Red, 7 Blue; 7 Blue, 7 Red; 13 Blue, 1 Green, 2 Red; 7 Green, 5 Red, 9 Blue
Game 51: 4 Blue, 9 Red, 1 Green; 16 Red; 2 Blue, 6 Red; 11 Red, 6 Blue
Game 52: 4 Green, 4 Blue, 9 Red; 5 Blue, 4 Red, 16 Green; 16 Green, 3 Red
Game 53: 2 Green, 12 Red; 2 Red, 5 Green, 15 Blue; 9 Blue, 17 Red, 9 Green; 2 Blue, 6 Red, 4 Green
Game 54: 2 Red, 3 Blue, 5 Green; 8 Green, 3 Blue; 9 Green, 3 Blue, 3 Red; 1 Blue, 4 Green
Game 55: 6 Green, 11 Blue, 12 Red; 10 Blue, 6 Red, 13 Green; 7 Green, 9 Blue; 10 Green, 20 Red, 7 Blue; 9 Green, 14 Red, 8 Blue; 14 Green, 15 Red
Game 56: 1 Green, 8 Red, 1 Blue; 1 Green, 3 Blue, 13 Red; 5 Red, 3 Blue; 5 Blue, 16 Red; 12 Red, 4 Blue
Game 57: 7 Green, 5 Blue; 13 Blue; 1 Red, 11 Green, 4 Blue; 1 Red, 7 Green, 5 Blue
Game 58: 14 Blue, 6 Green, 9 Red; 7 Blue, 1 Green, 11 Red; 3 Red, 9 Blue, 6 Green; 4 Green, 2 Red; 2 Blue, 6 Green; 11 Blue, 1 Red
Game 59: 6 Red, 1 Blue, 5 Green; 4 Green; 15 Green; 7 Red, 1 Blue, 12 Green; 7 Red, 1 Blue, 3 Green
Game 60: 3 Blue, 6 Red, 2 Green; 7 Green, 6 Red, 4 Blue; 3 Green, 1 Red, 4 Blue; 3 Red, 1 Green; 9 Red, 5 Green, 4 Blue
Game 61: 1 Green, 3 Blue; 1 Red, 2 Green; 1 Green, 2 Blue, 2 Red
Game 62: 10 Green, 15 Blue, 14 Red; 11 Blue, 11 Red, 16 Green; 5 Red, 5 Green, 12 Blue
Game 63: 2 Blue, 5 Red; 7 Blue, 2 Green, 2 Red; 2 Red, 1 Blue
Game 64: 9 Blue, 12 Red, 4 Green; 5 Blue, 13 Red; 1 Red, 2 Green, 7 Blue
Game 65: 4 Blue, 8 Red; 13 Green, 8 Blue, 5 Red; 1 Green, 5 Blue, 7 Red; 11 Red, 7 Blue, 10 Green
Game 66: 8 Red, 17 Blue; 1 Green, 9 Red, 7 Blue; 12 Red
Game 67: 14 Blue, 12 Green, 3 Red; 12 Green; 9 Green, 13 Red, 15 Blue; 2 Red, 10 Green, 1 Blue
Game 68: 11 Blue, 14 Green; 14 Green; 9 Blue, 7 Green, 1 Red; 9 Blue, 7 Green; 17 Green, 2 Blue; 4 Green, 4 Blue
Game 69: 4 Blue, 14 Green, 6 Red; 11 Red, 7 Green, 10 Blue; 4 Red, 8 Blue, 8 Green; 7 Green, 6 Red, 7 Blue
Game 70: 12 Red, 16 Green, 11 Blue; 16 Green, 15 Blue, 5 Red; 10 Blue, 1 Red, 12 Green; 9 Red, 8 Blue, 4 Green; 2 Green, 8 Red, 3 Blue
Game 71: 8 Red, 1 Blue, 5 Green; 12 Green, 7 Red; 11 Green, 1 Blue, 7 Red
Game 72: 5 Green, 15 Red; 7 Green, 3 Red, 4 Blue; 10 Red, 1 Green; 6 Blue, 15 Red, 3 Green
Game 73: 1 Green, 5 Red, 1 Blue; 6 Red, 3 Blue, 6 Green; 11 Red, 1 Blue
Game 74: 5 Red; 1 Blue, 3 Green, 3 Red; 2 Green, 7 Red; 1 Blue, 2 Red; 3 Red, 1 Green
Game 75: 13 Blue, 20 Red, 10 Green; 3 Green, 5 Blue, 14 Red; 9 Red, 13 Green, 7 Blue; 1 Blue, 15 Red, 2 Green; 11 Blue, 2 Green, 17 Red; 11 Red, 13 Blue, 13 Green
Game 76: 9 Red, 7 Green, 2 Blue; 7 Red, 2 Blue, 8 Green; 4 Blue, 3 Red, 9 Green; 4 Red, 1 Green; 1 Red, 2 Green, 3 Blue
Game 77: 5 Red, 2 Green, 15 Blue; 12 Green, 4 Red, 2 Blue; 10 Blue, 6 Red, 9 Green; 7 Blue, 3 Green; 16 Blue, 4 Red, 5 Green
Game 78: 11 Blue, 3 Green, 19 Red; 3 Blue, 1 Red; 8 Red, 14 Blue, 3 Green; 8 Blue, 8 Green, 16 Red; 8 Blue, 14 Red; 12 Blue, 11 Red, 2 Green
Game 79: 10 Blue, 5 Red, 1 Green; 3 Blue, 13 Red; 15 Red, 1 Green; 4 Red, 6 Blue, 1 Green; 1 Green, 6 Blue
Game 80: 7 Red, 1 Green, 1 Blue; 1 Blue, 4 Red, 3 Green; 2 Red, 2 Green; 7 Red, 1 Blue, 1 Green; 2 Red, 1 Green, 3 Blue
Game 81: 12 Green, 2 Red, 8 Blue; 1 Green, 1 Blue, 1 Red; 7 Blue, 1 Red, 11 Green; 1 Red, 12 Blue, 4 Green
Game 82: 18 Red, 5 Blue, 4 Green; 6 Green, 11 Red; 11 Green, 18 Red, 5 Blue; 4 Green, 17 Red, 4 Blue; 5 Blue, 14 Red, 15 Green
Game 83: 4 Red, 6 Blue, 6 Green; 9 Red, 4 Green; 8 Green, 7 Blue; 2 Blue, 9 Red, 13 Green; 2 Blue, 9 Green, 11 Red
Game 84: 15 Blue; 4 Green, 1 Red, 15 Blue; 2 Green, 16 Blue; 3 Green, 14 Blue; 16 Blue
Game 85: 3 Red, 7 Green, 8 Blue; 3 Blue, 17 Green, 7 Red; 13 Green, 4 Blue; 6 Blue, 8 Green
Game 86: 16 Green, 6 Blue; 12 Blue, 9 Red, 11 Green; 17 Green, 4 Blue, 8 Red
Game 87: 6 Blue, 3 Green, 13 Red; 13 Blue; 12 Red, 2 Green, 1 Blue
Game 88: 6 Red, 2 Blue; 16 Red, 13 Blue, 1 Green; 2 Green, 11 Blue, 2 Red; 12 Blue, 9 Red, 1 Green; 5 Blue, 2 Red, 2 Green; 18 Red, 3 Blue
Game 89: 6 Green, 5 Blue; 4 Green, 4 Blue; 3 Red, 5 Blue
Game 90: 3 Green, 8 Blue; 2 Green, 7 Blue, 9 Red; 8 Red, 2 Blue, 4 Green; 1 Green, 3 Red, 7 Blue; 4 Blue, 4 Green, 2 Red; 9 Red, 3 Blue, 3 Green
Game 91: 9 Red, 12 Green, 1 Blue; 11 Green, 9 Red, 2 Blue; 1 Blue, 8 Red, 4 Green; 6 Red, 9 Green; 2 Blue, 10 Red, 1 Green; 2 Blue, 15 Green, 13 Red
Game 92: 3 Green, 11 Red, 16 Blue; 8 Blue, 1 Red, 6 Green; 4 Green, 1 Red, 5 Blue
Game 93: 9 Blue, 3 Red, 13 Green; 2 Red, 9 Blue; 3 Blue, 17 Green, 5 Red; 4 Green, 8 Blue
Game 94: 2 Blue, 3 Red, 9 Green; 4 Blue, 1 Red, 6 Green; 8 Green, 2 Blue; 4 Green, 2 Blue, 7 Red
Game 95: 5 Green, 3 Blue; 4 Blue, 3 Green, 8 Red; 3 Green, 4 Red, 3 Blue; 2 Blue, 4 Red; 9 Blue, 5 Red, 3 Green
Game 96: 11 Green; 10 Green, 5 Blue, 11 Red; 5 Blue, 13 Red, 15 Green; 10 Green, 1 Blue, 11 Red
Game 97: 5 Green, 6 Blue, 1 Red; 7 Green, 1 Red; 5 Blue; 3 Blue, 1 Red
Game 98: 1 Blue, 5 Green, 7 Red; 3 Red, 5 Green, 1 Blue; 4 Blue, 8 Green, 2 Red; 4 Green, 1 Blue, 6 Red
Game 99: 12 Blue, 8 Green; 2 Green; 3 Red, 7 Green, 5 Blue; 1 Green, 1 Blue, 2 Red
Game 100: 4 Blue, 14 Red; 12 Red, 1 Blue; 2 Red, 2 Blue; 8 Red; 14 Red, 2 Blue, 1 Green; 3 Blue";
