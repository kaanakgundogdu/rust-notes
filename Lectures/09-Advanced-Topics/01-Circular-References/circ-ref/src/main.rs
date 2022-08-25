
struct Student{
    name:String
}
impl Student {
    fn get_courses(&self,platform:Platform) ->Vec<String>{
        platform.enrollments.iter()
        .filter(|&e| e.student.name == self.name)
        .map(|e| e.course.name.clone())
        .collect()
    }
}
struct Course{
    name:String
}

struct Enrollment<'a>{
    student: &'a Student,
    course: &'a Course
}

impl<'a> Enrollment<'a> {
    fn new(student:&'a Student,course:&'a Course) -> Enrollment<'a>{
        Enrollment { student: student, course: course }
    }
}
struct Platform<'a>{
    enrollments:Vec<Enrollment<'a>>
}
impl <'a> Platform<'a> {
    fn new()->Platform<'a> {
        Platform { enrollments: Vec::new() }
    }
    fn enroll(&mut self, student:&'a Student, course:&'a Course) {
        self.enrollments.push(Enrollment::new(student, course))
    }
}

fn main() {
    let kaan=Student{name:"Kaan".into()};
    let rust_course=Course{name:"Learning Rust".into()};
    let web_course=Course{name:"Web Dev.".into()};
    let cpp_course=Course{name:"C++".into()};
    let mut pl=Platform::new();
    pl.enroll(&kaan, &rust_course);
    pl.enroll(&kaan, &web_course);
    pl.enroll(&kaan, &cpp_course);

    for c in kaan.get_courses(pl) {
        println!("Kaan is taking {} course",c)
    }

}
