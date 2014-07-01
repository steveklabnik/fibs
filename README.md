# Fibs

Thanks @brianloveswords :heart:

```bash
$ cargo build
$ ./target/fibs
$ ./target/fibs2
$ ./target/fibs3
```

To benchmark:

Cargo doesn't have a bench command yet, so you have to fudge it:

```bash
$ cargo test
$ target/tests/fibs1 --bench
$ target/tests/fibs2 --bench
$ target/tests/fibs3 --bench
```

On my system:

```
steve@computer:~/tmp/fibs$ target/tests/fibs1 --bench

running 1 test
test bench_fib_of_40000000 ... bench:       624 ns/iter (+/- 10)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

steve@computer:~/tmp/fibs$ target/tests/fibs2 --bench

running 1 test
test bench_fib_of_40000000 ... bench:       705 ns/iter (+/- 10)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

steve@computer:~/tmp/fibs$ target/tests/fibs3 --bench

running 1 test
test bench_fib_of_40000000 ... bench:      1194 ns/iter (+/- 31)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured

```

I am not sure why the last version is slower...
