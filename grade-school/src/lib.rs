use std::collections::BTreeMap;

#[derive(Default)]
pub struct School {
    grade_map: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grade_map: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grade_map
            .entry(grade)
            .or_insert(Vec::new())
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grade_map.keys().map(|&x| x).collect::<Vec<u32>>()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.grade_map.get(&grade) {
            Some(v) => {
                let mut grade = v.to_vec();
                grade.sort();
                Some(grade)
            }
            None => None,
        }
    }
}
