pub fn reply(message: &str) -> &str {
    let is_yelling = !(message.chars().any(|c: char| c.is_lowercase())
        || message.chars().all(|c| !c.is_alphabetic()));

    if message.trim().ends_with("?") {
        match is_yelling {
            true => "Calm down, I know what I'm doing!",
            false => "Sure.",
        }
    } else if message.find(|c: char| c.is_alphanumeric()).is_none() {
        return "Fine. Be that way!";
    } else {
        match is_yelling {
            true => "Whoa, chill out!",
            false => "Whatever.",
        }
    }
}
