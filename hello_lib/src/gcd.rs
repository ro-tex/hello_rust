// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn gcd_euclid(a: u64, b: u64) -> u64 {
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

// https://en.wikipedia.org/wiki/Binary_GCD_algorithm
fn gcd_bin(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = (a, b);
    let mut gcd = 1;
    while x != 0 && y != 0 {
        if x % 2 == 0 && y % 2 == 0 {
            gcd *= 2;
            x /= 2;
            y /= 2;
        } else if x % 2 == 0 {
            x /= 2;
        } else if y % 2 == 0 {
            y /= 2;
        } else {
            if x >= y {
                x = (x - y) / 2;
            } else {
                y = (y - x) / 2;
            }
        }
    }
    return if x == 0 { y * gcd } else { x * gcd };
}
