extern crate math;

#[test]
fn part1_smoke() {
    assert!(is_valid(111111));
    assert!(!is_valid(223450));
    assert!(!is_valid(123789));
    assert!(!is_valid(555550));
    assert!(is_valid(555559));
    assert!(!is_valid(0));
}

#[test]
fn part1() {
    let ans = (402328..=864247).filter(|&n| is_valid(n)).count();
    assert_eq!(ans, 454);
}

#[test]
fn part2_smoke() {
    assert!(is_valid_2(112233));
    assert!(!is_valid_2(123444));
    assert!(is_valid_2(111122));
}

#[test]
fn part2() {
    let ans = (402328..=864247).filter(|&n| is_valid_2(n)).count();
    assert_eq!(ans, 288);
}

fn is_valid(n: u32) -> bool {
    let mut ds = math::digits(n);
    let mut prev = match ds.next() {
        Some(d) => d,
        None => return false,
    };
    let mut has_duplicate = false;
    for d in ds {
        if d > prev {
            return false;
        }
        if d == prev {
            has_duplicate = true;
        }
        prev = d;
    }
    has_duplicate
}

fn is_valid_2(n: u32) -> bool {
    let mut ds = math::digits(n);
    let mut count = 1;
    let mut prev = match ds.next() {
        Some(d) => d,
        None => return false,
    };
    let mut has_duplicate = false;
    for d in ds {
        if d > prev {
            return false;
        } else if d == prev {
            count += 1;
        } else {
            if count == 2 {
                has_duplicate = true;
            }
            prev = d;
            count = 1;
        }
    }
    has_duplicate || count == 2
}
