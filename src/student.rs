use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};


/*
    Implement the default trait for this object. 
    The default values should be name="{StudentName}", id="StudentId", date_of_birth=1985-01-01, score=0
    The usage will be as follows:
    let mut student_a = Student::default();
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Student {
    name: String,
    id: String,
    date_of_birth: Date<Utc>,
    pub score: u32
}

impl Student {
    /*
     You are required to finish the implementation of this method. 
     DO NOT change the parameters of the function. 
     This method creates a new instance of the Student object so that it can be used as follows,
     
     let mut student_a = Student::new("Alexis Murphy", "s-0232-123/233", "1996-12-3");
     
     */
    pub fn new(name: &str, id: &str, dob: &str) -> Self{
        let naive_date = DateTime::parse_from_str(dob, "YYYY-mm-dd")
            .unwrap()
            .date_naive();
        Student {
            name: name.to_string(),
            id: id.to_string(),
            // Result -> OK(), Err()
            date_of_birth: Date::from_utc(naive_date,Utc),
            score: 0
        }
    }
   
    pub fn set_score(&mut self, score: u32) {
        self.score = score;
    }
}

impl Default for Student {
    fn default() -> Self {
        Student {
            name: "{StudentName}".to_string(),
            id: "{StudentId}".to_string(),
            date_of_birth: Date::from_utc(
                NaiveDate::from_ymd(1985, 01, 01), Utc),
            score: 0
        }
    }
}
