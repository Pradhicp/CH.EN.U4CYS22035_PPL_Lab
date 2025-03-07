struct Student {
    name: String,
    age: u8,
    grade: char,
}
fn display_students(students: &Vec<Student>) {
    for student in students {
        println!("Name: {}, Age: {}, Grade: {}", student.name, student.age, student.grade);
    }
}
fn update_grade(students: &mut Vec<Student>, student_name: &str, new_grade: char) {
    for student in students.iter_mut() {
        if student.name == student_name {
            student.grade = new_grade;
            println!("Updated {}'s grade to {}", student.name, new_grade);
            return;
        }
    }
    println!("Student {} not found!", student_name);
}

fn main() {
    let mut students: Vec<Student> = vec![
        Student { name: String::from("Alice"), age: 20, grade: 'A' },
        Student { name: String::from("Bob"), age: 21, grade: 'B' },
        Student { name: String::from("Charlie"), age: 19, grade: 'C' },
    ];
    println!("Student Records:");
    display_students(&students);
    update_grade(&mut students, "Bob", 'A');
    println!("\nUpdated Student Records:");
    display_students(&students);
}

