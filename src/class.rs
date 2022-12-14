use crate::student::Student;

#[derive(Debug, Clone)]
pub struct Class {
    id: String,
    students: Vec<Student>,
}

impl Class {
    pub fn new(id: &str)->Self{
        Class {
            id: id.to_string(),
            students: Vec::new()
        }
    }
    
    /*
    Finish implementing this function. After adding the new student 
    to the students list, we should sort the students by the score 
    that they have in descending order. This means that the first 
    student should always be the one with the highest score. 
     */
    pub fn add_student(&mut self, student: Student) {
        // Add the student to the list of students.
        self.students.push(student);
        // Sort the students in descending order
        self.students.sort_by(|a, b| b.score.cmp(&a.score));
    }
    
    
}

// Implementing the Iterator trait on the class
impl Iterator for Class {
    type Item = Student;

    /*
    Finish this implementation for getting the next student in the class. It should be used in a loop,
    
    for student in class_a.next() {
        // Do something
    }
     */
    fn next(&mut self) -> Option<Self::Item> {
        self.students.iter().next().map(|x| x.clone())
    }
}