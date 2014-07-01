struct FibIter {
previous: int,
              current: int
}

fn fib_iter() -> FibIter {
    FibIter { previous: 1, current: 1 }
}

impl Iterator<int> for FibIter {
    fn next(&mut self) -> Option<int> {
        let tmp = self.previous;
        self.previous = self.current;
        self.current += tmp;
        Some(tmp)
    }
}

fn main() {
    let sum = fib_iter()
        .take_while(|&i| i < 4000000)
        .filter(|&i| i % 2 == 0)
        .fold(0, |acc, i| acc + i);
    println!("sum = {}", sum);
}
