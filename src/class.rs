use crate::student::Student;

pub struct Class {
    students: Vec<Student>,
    id: String
}

impl Class {
    pub fn new(id: &str)->Self{
        Class {
            students: Vec::new(),
            id: id.to_string()
        }
    }
    
    /*
    Finish implementing this function. After adding the new student 
    to the students list, we should sort the students by the score 
    that they have in descending order. This means that the first 
    student should always be the one with the highest score. 
     */
    pub fn add_student(&self, student: Student) {
        todo!()
    }
    
    
}

impl Iterator for Class {
    type Item = Student;

    /*
    Finish this implementation for getting the next student in the class. It should be used in a loop,
    
    for student in class_a.next() {
        // Do something
    }
     */
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}