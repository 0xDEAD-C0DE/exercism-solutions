use std::collections::HashMap;
pub struct School {
    students: HashMap<String,u32>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.students.contains_key(&student.to_string())  {
            println!("{} already in school", student);
        } else {
            self.students.insert(student.to_string(), grade);
        } 
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut g = vec![];
        for grade in self.students.values() {
            if !g.contains(grade) {
                g.push(*grade)
            }

        }
        g.sort_unstable();
        g

    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut stud: Vec<String> = vec![];
        for (s,g) in &self.students {
            if grade == *g {
                stud.push(s.to_string());
            }

        }
    stud.sort_unstable();
    stud
    }
}