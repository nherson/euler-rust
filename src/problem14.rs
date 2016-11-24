/*
Problem description

The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.

Link: https://projecteuler.net/problem=14
*/

use std::collections::HashMap;
use std::collections::hash_map::Iter;

// A Collatz
struct Collatz {
    map: HashMap<u64,u64>,
}

// Implement Collatz series length function that uses a `HashMap` for memoization
impl Collatz {
    fn new() -> Collatz {
        let mut m = HashMap::new();
        m.insert(1, 1);
        Collatz{ map: m }
    }
    fn length(&mut self, n: u64) -> u64 {
        // If we haven't computed the length of this number's Collatz series, do it;
        // computed using a recursive call and memoized into the `HashMap`
        // I _wanted_ to do the insert inside this block under the `None` case,
        // but the borrow checker wouldn't allow this because the scope is already performing
        // an immutable borrow because of the `self.map.get(&n)`. Therefore, just grab a true or
        // false in this match, after which the borrow goes out of scope and ends, then check
        // the bool to see if we need to do a `self.map.insert`.  Don't modify a data structure
        // in the same block as where we are looking into it!!!

        let computed: bool = match self.map.get(&n) {
            None => false,
            Some(_x) => true,
        };
        let mut _recursive_length: u64 = 0;
        if !computed {
            if n%2==0 {
                _recursive_length = self.length(n/2)
            } else {
                _recursive_length = self.length(n*3+1)
            }
            self.map.insert(n, _recursive_length+1);
        }
        // Get the result from the `HashMap` and return it
        match self.map.get(&n) {
            None => panic!("An error occurred!"),
            Some(x) => *x,
        }
    }
    fn iter(&self) -> Iter<u64,u64> {
        return self.map.iter();
    }
}

pub fn run() {
    println!("Problem 14!");

    let mut c = Collatz::new();

    let mut _noop: u64;
    for i in 1..1000000 {
        _noop = c.length(i)
    }

    let mut max_chain: u64 = 0;
    let mut max_chain_num: u64 = 0;
    for (num, chain_length) in c.iter() {
        if *num < 1000000 && *chain_length > max_chain {
            max_chain = *chain_length;
            max_chain_num = *num;
        }
        if *num == 999999 {
            println!("{}", *chain_length)
        }
    }
    println!("Max chain number: {} with chain length: {}", max_chain_num, max_chain);

}
