use std::fs;

#[test]
fn part1_smoke() {
    let mut machine = IntcodeMachine::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    machine.step();
    assert_eq!(machine.tape[3], 70);
}

#[test]
fn part1() {
    let contents = fs::read_to_string("days/day02/input.txt").expect("read input file");
    let tape = contents
        .split(',')
        .map(|s| s.trim().parse::<usize>().expect("parse usize"))
        .collect();
    let mut machine = IntcodeMachine::new(tape);

    machine.tape[1] = 12;
    machine.tape[2] = 2;
    assert_eq!(machine.run(), 4576384);
}

#[test]
fn part2() {
    let contents = fs::read_to_string("days/day02/input.txt").expect("read input file");
    let tape = contents
        .split(',')
        .map(|s| s.trim().parse::<usize>().expect("parse usize"))
        .collect();

    let (a, b) = reverse_engineer(tape, 19690720).expect("find answer");
    assert_eq!(100 * a + b, 5398);
}

struct IntcodeMachine {
    tape: Vec<usize>,
    pos: usize,
}

impl IntcodeMachine {
    fn new(tape: Vec<usize>) -> IntcodeMachine {
        IntcodeMachine { tape, pos: 0 }
    }
    fn step(&mut self) -> State {
        let op = Opcode::from_usize(self.tape[self.pos]).expect("invalid opcode");
        match op {
            Opcode::Halt => return State::Done,
            Opcode::Add => self.add(),
            Opcode::Mul => self.mul(),
        }
        self.pos += 4;
        State::Continue
    }
    fn add(&mut self) {
        let i = self.tape[self.pos + 1];
        let j = self.tape[self.pos + 2];
        let k = self.tape[self.pos + 3];
        self.tape[k] = self.tape[i] + self.tape[j];
    }
    fn mul(&mut self) {
        let i = self.tape[self.pos + 1];
        let j = self.tape[self.pos + 2];
        let k = self.tape[self.pos + 3];
        self.tape[k] = self.tape[i] * self.tape[j];
    }
    fn run(&mut self) -> usize {
        while self.step() == State::Continue {}
        self.tape[0]
    }
}

fn reverse_engineer(tape: Vec<usize>, target: usize) -> Option<(usize, usize)> {
    for a in 0..190 {
        for b in 0..100 {
            let mut machine = IntcodeMachine::new(tape.clone());
            machine.tape[1] = a;
            machine.tape[2] = b;
            if machine.run() == target {
                return Some((a, b));
            }
        }
    }
    None
}

enum Opcode {
    Add,
    Mul,
    Halt,
}

impl Opcode {
    fn from_usize(v: usize) -> Option<Opcode> {
        match v {
            1 => Some(Opcode::Add),
            2 => Some(Opcode::Mul),
            99 => Some(Opcode::Halt),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum State {
    Done,
    Continue,
}
