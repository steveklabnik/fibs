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
    let mut sum = 0;
    for i in fib_iter() {
        if i > 4000000 {
            break;
        }
        if i % 2 == 0 {
            sum += i;
        }
    }
    println!("sum = {}", sum);
}
