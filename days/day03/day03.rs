extern crate files;

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[test]
fn part1_smoke() {
    let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72"
        .parse()
        .expect("parse wire");
    let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83"
        .parse()
        .expect("parse wire");
    assert_eq!(closest_crossing(wire1, wire2), Some(159));
}

#[test]
fn part1() {
    let contents = files::read!("input.txt");
    let mut lines = contents.lines();
    let wire1 = lines.next().expect("line 1").parse().expect("parse wire");
    let wire2 = lines.next().expect("line 2").parse().expect("parse wire");
    assert_eq!(closest_crossing(wire1, wire2), Some(651));
}

#[test]
fn part2_smoke() {
    let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72"
        .parse()
        .expect("parse wire");
    let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83"
        .parse()
        .expect("parse wire");
    assert_eq!(earliest_crossing(wire1, wire2), Some(610));
}

#[test]
fn part2() {
    let contents = files::read!("input.txt");
    let mut lines = contents.lines();
    let wire1 = lines.next().expect("line 1").parse().expect("parse wire");
    let wire2 = lines.next().expect("line 2").parse().expect("parse wire");
    assert_eq!(earliest_crossing(wire1, wire2), Some(7534));
}

fn closest_crossing(w1: Wire, w2: Wire) -> Option<usize> {
    let w1: HashSet<Point> = w1.path().into_iter().collect();
    let w2: HashSet<Point> = w2.path().into_iter().collect();
    w1.intersection(&w2).map(|p| p.l0()).min()
}

fn earliest_crossing(w1: Wire, w2: Wire) -> Option<usize> {
    let path1: HashMap<Point, usize> = w1
        .path()
        .into_iter()
        .enumerate()
        .map(|(idx, p)| (p, idx))
        .collect();
    w2.path()
        .into_iter()
        .enumerate()
        .flat_map(|(i2, pt)| {
            path1
                .get(&pt)
                .map(|i1| i1 + i2 + 2 /* enumerate is 0-indexed */)
        })
        .min()
}

struct Wire {
    segments: Vec<Segment>,
}
struct Segment {
    len: usize,
    dir: Direction,
}
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Point(i32, i32);
impl Point {
    fn new() -> Point {
        Point(0, 0)
    }
    fn l0(&self) -> usize {
        self.0.abs() as usize + self.1.abs() as usize
    }
    fn step(&mut self, d: Direction) {
        match d {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
        }
    }
}

impl FromStr for Wire {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = vec![];
        for seg in s.trim().split(',') {
            if seg.len() < 2 {
                return Err(());
            }
            let dir = match &seg[0..1] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => return Err(()),
            };
            let len = seg[1..].parse().map_err(|_| ())?;
            segments.push(Segment { len, dir });
        }
        Ok(Wire { segments })
    }
}
impl Wire {
    fn path(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let mut p = Point::new();
        for segment in &self.segments {
            for _ in 0..segment.len {
                p.step(segment.dir);
                points.push(p);
            }
        }
        points
    }
}
