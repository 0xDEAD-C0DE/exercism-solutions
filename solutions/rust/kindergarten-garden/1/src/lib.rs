#![feature(array_chunks)]
pub fn get_student_id(student: &str) -> Option<usize> {
let students = vec![
    "Alice", "Bob", "Charlie", "David",
    "Eve", "Fred", "Ginny", "Harriet",
    "Ileana", "Joseph", "Kincaid", "Larry"];

    let mut id = 0;
    let mut found: bool = false;
    for (i, s) in students.iter().enumerate() {
        if *s == student {
            id = i;
            found = true;
        }
    }

    if !found {
        None
    } else {
        Some(id)
    }

}

pub fn get_plants(list:Vec< char>) -> Vec<&'static str> {
    let mut plants: Vec<&str> = vec![];
    for i in list {
        match i {
            'C' => plants.push("clover"),
            'G' => plants.push("grass"),
            'R' => plants.push("radishes"),
            'V' => plants.push("violets"),
            _ => plants.push(""),
        }
    }
    plants
    
}

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let v = _diagram.chars().filter(|x| *x != '\n').collect::<Vec<char>>();
    let chunk = v.array_chunks::<2>().collect::<Vec<&[char;2]>>();
    
    let index = match get_student_id(_student) {
        Some(v) => v,
        None => panic!("Student not found"),
    };

    let len = (chunk.len()) / 2;
    let row = chunk.get(index).unwrap();
    
    let row_1 = chunk.get(index + len).unwrap();
    let mut students_plants: Vec<char> = vec![];

    for i in *row {
        students_plants.push(*i);
    }
    for i in *row_1 {
        students_plants.push(*i);
    }

    get_plants(students_plants)
}