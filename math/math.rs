pub fn digits(n: u32) -> Digits {
    Digits(n)
}

pub struct Digits(u32);
impl Iterator for Digits {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            return None;
        }
        let output = self.0 % 10;
        self.0 /= 10;
        Some(output)
    }
}
