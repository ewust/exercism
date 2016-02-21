use std::collections::HashMap;


pub struct School {
    gradebook: HashMap<usize, Vec<String>>
}


impl School {
    pub fn new() -> School {
        School { gradebook: HashMap::new() }
    }

    pub fn grades(&self) -> Vec<usize> {
        let mut grade_nums: Vec<usize> = self.gradebook.keys().cloned().collect();
        grade_nums.sort();
        grade_nums
    }

    pub fn add(&mut self, grade: usize, name: &str) {
        let e = self.gradebook.entry(grade).or_insert(vec![]);
        e.push(name.to_string());
        e.sort();   // Maybe an always-sorted data structure would be better here
    }

    pub fn grade(&self, grade: usize) -> Option<&Vec<String>> {
        self.gradebook.get(&grade)
    }
}
