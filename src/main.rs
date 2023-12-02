#![allow(dead_code)]
mod day1;
mod day2;

pub trait FromLine {
    fn from_line(line: &str) -> Option<Self>
    where
        Self: Sized;
}

pub fn parse_lines<'a, T: FromLine + 'a>(data: &'a str) -> impl Iterator<Item = T> + 'a {
    data.lines().flat_map(T::from_line)
}

fn main() {
    day2::day2();
}
