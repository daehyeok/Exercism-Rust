pub fn square_of_sum(n: u32) -> u32 {
    let k = (n * (n + 1)) / 2;
    k * k
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    let mut v = vec![sum_of_squares(n), square_of_sum(n)];
    v.sort();
    v[1] - v[0]
}
