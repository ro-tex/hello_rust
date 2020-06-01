#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// TODO How to expose the funcs from the other files of the library?

/// Returns the GCD based on Euclid's algorithm.
pub fn gcd(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = (a, b);
    while x != 0 && y != 0 {
        if x > y {
            x = x - y
        } else {
            y = y - x
        }
    }
    return if x == 0 { y } else { x };
}
