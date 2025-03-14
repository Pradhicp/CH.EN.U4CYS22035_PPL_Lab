use std::io;

fn update_salary(employee: (u32, String, u32)) -> (u32, String, u32) {
    let (id, name, salary) = employee;
    let updated_salary = if salary < 50000 {
        (salary as f32 * 1.1) as u32 
    } else {
        salary
    };
    (id, name, updated_salary)
}

fn main() {
    let mut id_input = String::new();
    let mut name_input = String::new();
    let mut salary_input = String::new();

    println!("Enter Employee ID:");
    io::stdin().read_line(&mut id_input).expect("Failed to read input");
    let id: u32 = id_input.trim().parse().expect("Please enter a valid ID");

    println!("Enter Employee Name:");
    io::stdin().read_line(&mut name_input).expect("Failed to read input");
    let name = name_input.trim().to_string();

    println!("Enter Employee Salary:");
    io::stdin().read_line(&mut salary_input).expect("Failed to read input");
    let salary: u32 = salary_input.trim().parse().expect("Please enter a valid salary");

    let employee = (id, name, salary);
    let updated_employee = update_salary(employee);

    println!("Updated Employee Details: {:?}", updated_employee);
}

