use std::iter::once;
pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }

    list.windows(2)
        .map(|part| format!("For want of a {} the {} was lost.", part[0], part[1]))
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .collect::<Vec<String>>()
        .join("\n")
}
