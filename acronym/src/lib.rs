pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphabetic() && c != '\'')
        .flat_map(|s| {
            s.chars()
                .take(1)
                .chain(
                    s.chars()
                        .skip_while(|ch| ch.is_ascii_uppercase())
                        .filter(|ch| ch.is_ascii_uppercase()),
                )
                .map(|ch| ch.to_ascii_uppercase())
        })
        .collect::<String>()
}
