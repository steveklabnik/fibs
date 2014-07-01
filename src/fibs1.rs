fn fib(pred: |x: int| -> bool) {
    let mut previous = 1;
    let mut current = 1;
    loop {
        if !pred(previous) { break }
        let tmp = current;
        current += previous;
        previous = tmp;
    }
}

/// uuuuuuuuugh https://github.com/rust-lang/rust/issues/12327
#[cfg(not(test))]
fn main() {
    let mut sum = 0;
    fib(|i| -> bool {
            if i > 4000000 {
            // more or less a `break`
            return false
            }

            if i % 2 != 0 {
            // more or less `next`
            return true
            }

            println!("adding {}", i);
            sum += i;
            true
            });
    println!("sum = {}", sum)
}


#[cfg(test)]
mod test {
    extern crate test;
    use self::test::Bencher;

    use super::fib;

    #[bench]
    fn bench_fib_of_40000000(b: &mut Bencher) {
        b.iter(|| {
          let mut sum = 0;
          fib(|i| -> bool {
            if i > 4000000 {
              // more or less a `break`
              return false
            }

            if i % 2 != 0 {
              // more or less `next`
              return true
            }

            sum += i;

            true
          });
        });
    }
}
