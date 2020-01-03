extern crate files;

const HEIGHT: usize = 6;
const WIDTH: usize = 25;
const SIZE: usize = WIDTH * HEIGHT;

#[test]
fn part1() {
    let input = files::read!("input.txt");
    let layers: Vec<Vec<u8>> = input
        .trim()
        .as_bytes()
        .chunks(SIZE)
        .map(|bs| bs.to_vec())
        .collect();
    let best = layers
        .into_iter()
        .min_by_key(|layer| layer.iter().filter(|&&b| b == b'0').count())
        .unwrap();
    let ones = best.iter().filter(|&&b| b == b'1').count();
    let twos = best.iter().filter(|&&b| b == b'2').count();
    assert_eq!(ones * twos, 2193);
}

#[test]
fn part2() {
    let input = files::read!("input.txt");
    let layers: Vec<Vec<u8>> = input
        .trim()
        .as_bytes()
        .chunks(SIZE)
        .map(|bs| bs.to_vec())
        .collect();
    let stacked: Vec<u8> = (0..SIZE)
        .map(|i| first(layers.iter().map(|layer| layer[i])))
        .collect();
    for line in stacked.chunks_exact(WIDTH) {
        printline(line)
    }
    // This prints out "YEHEF"
    panic!()
}

fn first(xs: impl Iterator<Item = u8>) -> u8 {
    for x in xs {
        if x != b'2' {
            return x;
        }
    }
    b'2'
}

fn printline(line: &[u8]) {
    for &b in line {
        match b {
            b'0' => print!("\x1B[47m "),
            b'1' => print!("\x1B[40m "),
            _ => print!("\x1B[0m "),
        };
    }
    println!("\x1B[0m");
}
