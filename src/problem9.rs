/*
Problem description

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a2 + b2 = c2
For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

pub fn run() {
    println!("Problem 9!");

    // a, b, c make up a pythagorean triple.  The difference between 2 consecutive squares, k^2 and
    // (k+1)^2 is 2k+1.
    //
    // Pseudocode:
    // Start with a=3, b=4, c=5
    // While 1000 % a+b+c != 0
    //   a = a + 2  # Get the next odd square for `a`
    //   While c^2-b^2 != a^2   # Find the next consecutive `b` and `c` whose difference is `a^2`
    //     b++ ; c++
    // factor = 1000/(a+b+c)
    // a = factor*a
    // b = factor*b
    // c = factor*c
    // print(a*b*c)  # or skip the last 4 steps and print a*b*c*factor^3
    //
    // if the above, doesn't work, increment the distance between `b` and `c` by one, and do it all
    // over again. If c-b is odd, start with a=3, else start with a=2

    // separator; distance between b and c to iterate over
    let mut s: i32 = 1;
    let mut a: i32 = 3;
    let mut b: i32 = 4;
    let mut c: i32 = b+s;

    'reset: loop {
        'inc_a: while 1000 % (a+b+c) != 0 {
            // Get the next odd/even square
            a = a + 2;
            // Find the difference of two consecutive squares (which will always be odd/even depending on `s`) that
            // matches the next odd square above (`a^2`)
            'inc_bc: while c.pow(2)-b.pow(2) != a.pow(2) {
                // Slide our consecutive squares over by 1
                b = b + 1;
                c = c + 1;
                // Beyond the point of no return so bail out, reset `b` and `c` increment `a`
                if a+b+c > 1000 {
                    b=4;
                    c=b+s;
                    continue 'inc_a;
                }
            }
            // are we done?
            if 1000%(a+b+c) == 0 {
                break 'reset;
            }
        }
        // reset everything
        s = s + 1;
        b = 4;
        c = b+s;
        if s%2 == 0 {
            a = 2;
        } else {
            a = 3;
        }
    }

    let factor = 1000 / (a+b+c);
    println!("a={}, b={}, c={}", a, b, c);
    println!("Pythagorean tripe, a*b*c={}", factor.pow(3)*a*b*c)
}
