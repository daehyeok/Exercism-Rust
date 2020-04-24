fn bottle_sentense(n: u32) -> String {
    let n_str = n.to_string();
    let parts = vec![
        if n == 0 { "No more" } else { &n_str },
        if n != 1 { "bottles" } else { "bottle" },
        "of beer",
    ];

    parts.iter().map(|&x| x).collect::<Vec<&str>>().join(" ")
}

fn action_sentense(n: u32) -> String {
    if n == 0 {
        return "Go to the store and buy some more".to_string();
    }

    format!(
        "Take {} down and pass it around",
        if n == 1 { "it" } else { "one" }
    )
}

pub fn verse(n: u32) -> String {
    let next_n = if n == 0 { 99 } else { n - 1 };

    let sentence = bottle_sentense(n);
    format!(
        "{} on the wall, {}.\n{}, {} on the wall.\n",
        sentence,
        sentence.to_lowercase(),
        action_sentense(n),
        bottle_sentense(next_n).to_lowercase()
    )
    .to_string()
}

pub fn sing(start: u32, end: u32) -> String {
    (end..(start + 1))
        .rev()
        .map(|i| verse(i))
        .collect::<Vec<String>>()
        .join("\n")
}
