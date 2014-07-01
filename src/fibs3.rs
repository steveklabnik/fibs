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

/// uuuuuuuuugh https://github.com/rust-lang/rust/issues/12327
#[cfg(not(test))]
fn main() {
    let sum = fib_iter()
        .take_while(|&i| i < 4000000)
        .filter(|&i| i % 2 == 0)
        .fold(0, |acc, i| acc + i);
    println!("sum = {}", sum);
}

#[cfg(test)]
mod test {

    extern crate test;
    use self::test::Bencher;

    use super::fib_iter;

    #[bench]
    fn bench_fib_of_40000000(b: &mut Bencher) {
        b.iter(|| {
          let sum = fib_iter()
              .take_while(|&i| i < 4000000)
              .filter(|&i| i % 2 == 0)
              .fold(0, |acc, i| acc + i);
        });
    }
}
