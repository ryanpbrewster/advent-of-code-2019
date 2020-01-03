extern crate files;

use std::convert::From;
use std::fmt;

const HEIGHT: usize = 6;
const WIDTH: usize = 25;
const SIZE: usize = WIDTH * HEIGHT;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Color {
    Black,
    White,
    Clear,
}
impl From<u8> for Color {
    fn from(b: u8) -> Color {
        match b {
            b'0' => Color::Black,
            b'1' => Color::White,
            _ => Color::Clear,
        }
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Color::White => f.write_str("\x1B[47m \x1B[0m"),
            Color::Black => f.write_str("\x1B[40m \x1B[0m"),
            Color::Clear => f.write_str(" "),
        }
    }
}

#[test]
fn part1() {
    let input = files::read!("input.txt");
    let layers: Vec<Vec<Color>> = input
        .as_bytes()
        .chunks_exact(SIZE)
        .map(|bs| bs.iter().map(|&b| Color::from(b)).collect())
        .collect();
    let best = layers
        .into_iter()
        .min_by_key(|layer| layer.iter().filter(|&&c| c == Color::Black).count())
        .unwrap();
    let whites = best.iter().filter(|&&c| c == Color::White).count();
    let clears = best.iter().filter(|&&c| c == Color::Clear).count();
    assert_eq!(whites * clears, 2193);
}

#[test]
fn part2() {
    let input = files::read!("input.txt");
    let layers: Vec<Vec<Color>> = input
        .as_bytes()
        .chunks_exact(SIZE)
        .map(|bs| bs.iter().map(|&b| Color::from(b)).collect())
        .collect();
    let stacked: Vec<Color> = (0..SIZE)
        .map(|i| first(layers.iter().map(|layer| layer[i])))
        .collect();
    for line in stacked.chunks_exact(WIDTH) {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    // This prints out "YEHEF"
    // panic!()
}

fn first(xs: impl Iterator<Item = Color>) -> Color {
    for x in xs {
        if x != Color::Clear {
            return x;
        }
    }
    Color::Clear
}
