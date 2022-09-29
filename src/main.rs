use std::collections::binary_heap::Iter;
use std::ops::Deref;
use rand::{Rng, thread_rng};
use crate::class::Class;
use crate::student::Student;

pub mod class;
pub mod student;

pub fn main() {
    let students = vec![ 
        Student::new("Alexis Murphy", "S-1239-232/199", "1996-2-12"),
        Student::new("Geoffrey Jones", "E-1011-892/088", "1994-12-19"),
        Student::new("Samantha Blake", "S-1239-111/102", "1997-10-7"),
        Student::new("Sean Winston", "B-0827-192/277", "1994-3-22")
    ];
    set_scores(students.clone());
    
    let ethics_class = Class::new("Ethics_2022");
    students.iter().for_each(
        |student| 
            ethics_class.add_student(student.clone())
    );
    
    println!("{:?}", ethics_class);
}

pub fn set_scores(students: Vec<Student>) {
    for mut student in students {
        student.set_score(thread_rng().gen_range(0..100));
    }
}
