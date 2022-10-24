// use crate::   //gcd::gcd_bin_a;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// Q: How to access functions from another file?
// A: Define it as a module.
mod gcd;

// TODO Q: How to expose the funcs from the other files of the library?
//      A: Add wrappers?

/// Returns the GCD based on Euclid's algorithm.
pub fn gcd_euclid(a: u64, b: u64) -> u64 {
    gcd::gcd_euclid(a, b)
}

/// Returns the GCD based on the Binary GCD algorithm.
pub fn gcd_bin(a: u64, b: u64) -> u64 {
    gcd::gcd_bin(a, b)
}
