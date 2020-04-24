use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&n, ch_vec)| ch_vec.iter().map(move |ch| (ch.to_ascii_lowercase(), n)))
        .collect::<BTreeMap<char, i32>>()
}
