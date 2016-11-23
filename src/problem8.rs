/*
Problem description

The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.

73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450

Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
*/

pub fn run() {
    println!("Problem 8!");

    let num_block = "\
    73167176531330624919225119674426574742355349194934\
    96983520312774506326239578318016984801869478851843\
    85861560789112949495459501737958331952853208805511\
    12540698747158523863050715693290963295227443043557\
    66896648950445244523161731856403098711121722383113\
    62229893423380308135336276614282806444486645238749\
    30358907296290491560440772390713810515859307960866\
    70172427121883998797908792274921901699720888093776\
    65727333001053367881220235421809751254540594752243\
    52584907711670556013604839586446706324415722155397\
    53697817977846174064955149290862569321978468622482\
    83972241375657056057490261407972968652414535100474\
    82166370484403199890008895243450658541227588666881\
    16427171479924442928230863465674813919123162824586\
    17866458359124566529476545682848912883142607690042\
    24219022671055626321111109370544217506941658960408\
    07198403850962455444362981230987879927244284909188\
    84580156166097919133875499200524063689912560717606\
    05886116467109405077541002256983155200055935729725\
    71636269561882670428252483600823257530420752963450";

    let window_size: usize = 13;

    // The largest product we have seen overall
    // It can get pretty big, so use a 64-bit unsigned integer to store the value
    let mut max_product: u64 = 0;

    // Array holding large digit window
    // Rust will not allow for dynamically sized arrays (I think it needs to be known at compile-time),
    // so here just specify some arbitrarily large value, but only use the first `window_size`
    // digits.
    let mut product_digits: [u64;100] = [0;100];

    // Get an iterator over the digits in the number block
    let block_chars = num_block.chars();

    // We are going to use `i` as a way to take modulo index into the array.
    // This essentiall makes `product_digits` a ring buffer holding the last `window_size` digits
    for (i, c) in block_chars.enumerate() {

        // Parse the current as a base 10 number
        let current_digit = match c.to_digit(10) {
            Some(x) => x,
            None => panic!("Error parsing digit: {}", c),
        };

        // Push the digit into the vector
        product_digits[i%window_size] = current_digit as u64;

        if i < window_size-1 {
            // First the first 12 iterations, we can't look at a product, so just continue
            continue
        } else if i == window_size {
            // Compute a product, but don't pop anything because there are exactly 13 elements;
            // in this case, unconditionally update `max_product` since it's the first one we look at
            max_product = compute_product(&product_digits[..window_size])
        } else {
            // Compute the product for the current window and compare to the current `max_product`
            let p = compute_product(&product_digits[..window_size]);
            if p > max_product {
                max_product = p;
            }
        }
    }
    println!("Max product for window size of {} is: {}", window_size, max_product)
}

// Takes a reference to a slice of u64 numbers, and a window size `n`, and multiplies the
// `n` items in the slice together
fn compute_product(digits: &[u64]) -> u64 {
    let mut product: u64 = 1;
    for i in 0..digits.len() {
        product = product * digits[i] as u64
    }
    product
}
