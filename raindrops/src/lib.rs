pub fn raindrops(n: u32) -> String {
    let mut sound = "".to_string();
    for (_, s) in [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|(i, _)| n % i == 0)
    {
        sound.push_str(&s.to_string());
    }

    match sound.len() {
        0 => n.to_string(),
        _ => sound,
    }
}
