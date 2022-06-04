use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub gender: String,
    pub age: u32,
}
impl Student {
    pub fn new_student(name: String, gender: String, age: u32) -> Student {
        Self {
            name: name,
            gender: gender,
            age: age,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Course {
    course_name: String,
    pub units: u32,
    students: Vec<Student>,
}
impl Course {
    fn new_course(course_name: String, units: u32) -> Course {
        Self {
            course_name: course_name,
            units: units,
            students: Vec::new(),
        }
    }

    fn add_student_to_course(&mut self, student: Student) {
        self.students.push(student)
    }
    fn view_students_in_course(&self) -> &Vec<Student> {
        &self.students
    }
}
#[derive(Debug, Clone)]
pub struct Department {
    courses: HashMap<String, Option<Course>>,
}
impl Department {
    fn new_dep(dep_name: String) -> Self {
        let mut dep = Self {
            courses: HashMap::new(),
        };
        dep.courses.insert(dep_name, None);
        dep
    }

    fn add_course_dep(&mut self, course: Course) {
        self.courses
            .insert(course.course_name.clone(), Some(course));
    }
    fn view_courses(&mut self) -> Vec<Course> {
        let mut v = Vec::new();
        let courses = &mut self.courses;
        courses.into_iter().for_each(|c| match c.1.as_ref() {
            Some(c) => {
                v.push(c.clone());
            }
            None => (),
        });

        v
    }
    fn add_new_student_to_dep_course(
        &mut self,
        student: Student,
        course_name: &String,
    ) -> Result<(), String> {
        match self.courses.get_mut(course_name) {
            Some(course) => {
                course.as_mut().unwrap().add_student_to_course(student);
                // let mut my_course = course.unwrap().add_student_to_course(student);
                // my_course.students.push(student);
                Ok(())
            }
            None => Err(format!("no such course ")),
        }
    }

    fn view_students_in_dep_corse(&mut self, course_name: &String) -> Option<&Vec<Student>> {
        match self.courses.get_mut(course_name) {
            Some(c) => {
                let students_in_course = c.as_mut().unwrap().view_students_in_course();
                Some(students_in_course)
            }
            None => None,
        }
    }
}
fn main() {
    let mut dep_it = Department::new_dep("it".to_string());
    // let mut students: Vec<Student> = Vec::new();
    let onchez = Student::new_student(String::from("onchez"), String::from("male"), 21);
    // students.push(onchez);
    let diana = Student::new_student(String::from("diana"), String::from("female"), 17);

    let bit = Course::new_course("bit".to_string(), 21);
    let course_name = bit.course_name.clone();
    dep_it.add_course_dep(bit);

    let courses = dep_it.view_courses();
    println!("{:#?}", courses);
    match dep_it.add_new_student_to_dep_course(onchez, &course_name) {
        Ok(_) => println!("added student succefully "),
        Err(e) => println!("{}", e),
    }
    match dep_it.add_new_student_to_dep_course(diana, &course_name) {
        Ok(_) => println!("added student succefully "),
        Err(e) => println!("{}", e),
    }

    let students_in_it = dep_it.view_students_in_dep_corse(&course_name);

    println!("students in {} are {:#?}", &course_name, students_in_it)
}
