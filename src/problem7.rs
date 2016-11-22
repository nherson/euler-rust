/*

Problem description

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?

*/

// The count of prime numbers that we want to find
static PRIMES_TARGET: usize = 10001;

pub fn run() {
    println!("Problem 7!");

    // Store all discovered primes in this vector
    let mut primes = vec![2];

    // start with 3 for finding primes, using 2 as a given
    let mut x = 3;

    while primes.len() != PRIMES_TARGET {
        // Don't need to pass a reference to `x` because it has the Copy trait, so it's
        // essentially pass by value
        // `primes` must pass a reference, so use `&` to let `is_prime` borrow a reference to
        // the vector
        if is_prime(x, &primes) {
            primes.push(x);
        }
        // The binding `x` is mutable, so this is valid
        x = x + 1;
    }
    println!("Prime number {} is: {}", PRIMES_TARGET, primes[PRIMES_TARGET-1])
}

// Returns true if `n` is prime, else false
// assumes that all items in `primes` are indeed prime numbers, and contains every prime
// number less than `n`
fn is_prime(n: i32, primes: &Vec<i32>) -> bool {
    for p in primes {
        if n % p == 0 {
            return false;
        }
    }
    true
}
