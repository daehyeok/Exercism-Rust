fn is_prime(n: u32) -> bool {
    if n < 4 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    !(5..((n as f64).sqrt() as u32 + 1))
        .step_by(6)
        .any(|i| n % i == 0 || n % (i + 2) == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..u32::max_value())
        .filter(|&k| is_prime(k))
        .skip(n as usize)
        .next()
        .unwrap()
}
