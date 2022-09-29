use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};


/*
    Implement the default trait for this object. The default values should be name="{StudentName}", id="StudentId", date_of_birth=1985-01-01, score=0
    The usage will be as follows:
    let mut student_a = Student::default();
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Student {
    name: String,
    id: String,
    date_of_birth: Date<Utc>,
    score: u32
}

impl Student {
    /*
     You are required to finish the implementation of this method. 
     DO NOT change the parameters of the function. 
     This method creates a new instance of the Student object so that it can be used as follows,
     
     let mut student_a = Student::new("Alexis Murphy", "s-0232-123/233", "1996-12-3");
     
     */
    pub fn new(name: &str, id: &str, dob: &str) -> Self{
        todo!()
    }
   
    pub fn set_score(&mut self, score: u32) {
        self.score = score;
    }
}
