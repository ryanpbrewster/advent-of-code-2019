use std::fs;

#[test]
fn part1() {
    let contents = fs::read_to_string("days/day01/input.txt").expect("read input file");
    let total_fuel: i32 = contents
        .lines()
        .map(|s| fuel_for_module(s.parse().expect("parse line as i32")))
        .sum();

    assert_eq!(total_fuel, 3305301);
}

#[test]
fn part2_smoke() {
    assert_eq!(meta_fuel_for_module(14), 2);
    assert_eq!(meta_fuel_for_module(1969), 966);
}

#[test]
fn part2() {
    let contents = fs::read_to_string("days/day01/input.txt").expect("read input file");
    let total_fuel: i32 = contents
        .lines()
        .map(|s| meta_fuel_for_module(s.parse().expect("parse line as i32")))
        .sum();

    assert_eq!(total_fuel, 4955106);
}

fn fuel_for_module(mass: i32) -> i32 {
    mass / 3 - 2
}

fn meta_fuel_for_module(mass: i32) -> i32 {
    let mut total = 0;
    let mut cur = mass;
    while cur > 0 {
        cur = std::cmp::max(fuel_for_module(cur), 0);
        total += cur;
    }
    total
}
