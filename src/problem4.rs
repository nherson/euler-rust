/*
Problem description:

A palindromic number reads the same both ways. The largest palindrome made from the product of
two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
pub fn run() {
    println!("Problem 4!");

    let mut max_palindrome = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            let n = x * y;
            if is_palindrome(&n) && n > max_palindrome {
                max_palindrome = n;
            }
        }
    }
    println!("Max palindrome: {}", max_palindrome)
}

// Returns true if the number is a palindrome, else false
fn is_palindrome(n: &i32) -> bool {
    let n_str = n.to_string();
    let n_chars = n_str.chars();
    let forward = n_chars;
    let mut backward = forward.clone().rev();

    for a in forward {
        let b = match backward.next() {
            Some(x) => x,
            None => panic!("Palindrome iterators are sized differently!")
        };
        if a != b {
            return false;
        }
    }
    true
}
