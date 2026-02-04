use std::io;
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Suspended,
    Graduated,
    Probation,
}

#[derive(Debug)]
struct Student {
    id: u8,
    name: String,
    age: u32,
    status: Status,
}
#[derive(Debug)]
struct StudentList {
    data: Vec<Student>,
    next_id: u8,
}

impl StudentList {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            next_id: 1,
        }
    }

    fn add_student(&mut self, name: String, age: u32) -> u8 {
        let current_id = self.next_id;

        let student = Student {
            id: current_id,
            name,
            age,
            status: Status::Active,
        };
        self.data.push(student);
        self.next_id += 1;
        current_id
    }

    fn get_all_students(&self) -> &Vec<Student> {
        &self.data
    }

    fn get_single_student(&self, id: u8) -> Option<&Student> {
        self.data.iter().find(|student| student.id == id)
    }

    fn update_status(&mut self, id: u8, new_status: Status) {
        if let Some(student) = self.data.iter_mut().find(|s| s.id == id) {
            student.status = new_status;
        }
    }

    fn graduate_student(&mut self, id: u8) {
        self.update_status(id, Status::Graduated);
    }

    fn delete_student(&mut self, id: u8) {
        self.data.retain(|student| student.id != id);
    }
}

fn main() {
    println!("Welcome to the school");

    println!("Add new student");

    let mut students = StudentList::new();
    //   println!("COmmands:");
    //     println!("View: to see all students");
    //     println!("Add: to add student");
    //     println!("Del: to delete student");
    //     println!("Update: to update student status");
    //     println!("Graduate: to graaduate student");
    //     println!("Exit: to exit");

    let mut command = String::new();

    loop {
        println!("\nCommands:");
        println!("View: to see all students");
        println!("Add: to add student");
        println!("Del: to delete student");
        println!("Update: to update student status");
        println!("Graduate: to graduate student");
        println!("Exit: to exit");

        // Get command input at the beginning of each loop iteration
        take_command(&mut command);

        // DEBUG: Print current command value
        println!("DEBUG: Current command = '{}'", command);

        if command == "view" {
            println!("viewing................");

            if students.get_all_students().is_empty() {
                println!("No students found. Please add students.");
            } else {
                for student in students.get_all_students() {
                    println!(
                        "ID: {}, Name: {}, Age: {}, Status: {:?}",
                        student.id, student.name, student.age, student.status
                    );
                }
            }
            // DEBUG: Print message after viewing students
            println!("DEBUG: Viewed students successfully");
        } else if command == "del" {
            let mut student_id = String::new();

            println!("Enter student ID to delete:");
            io::stdin()
                .read_line(&mut student_id)
                .expect("Failed to read line");

            students.delete_student(student_id.trim().parse().unwrap());
        } else if command == "add" {
            println!("Enter student details:");
            let mut student_name = String::new();
            let mut student_age = String::new();

            println!("Enter student name:");
            io::stdin()
                .read_line(&mut student_name)
                .expect("Failed to read line");

            println!("Enter student age:");
            io::stdin()
                .read_line(&mut student_age)
                .expect("Failed to read line");

            let age: u32 = student_age.trim().parse().unwrap();

            students.add_student(student_name.trim().to_string(), age);
            // DEBUG: Print message after adding student
            println!("DEBUG: Student added successfully");
        } else if command == "update" {
            let mut student_id = String::new();
            let mut new_status = String::new();

            println!("Enter student ID:");
            io::stdin()
                .read_line(&mut student_id)
                .expect("Failed to read line");

            println!("Enter new status (Active, Inactive, Suspended, Graduated, Probation):");
            io::stdin()
                .read_line(&mut new_status)
                .expect("Failed to read line");

            let status = match new_status.trim() {
                "Active" => Status::Active,
                "Inactive" => Status::Inactive,
                "Suspended" => Status::Suspended,
                "Graduated" => Status::Graduated,
                "Probation" => Status::Probation,
                _ => {
                    println!("Invalid status. Please try again.");
                    continue;
                }
            };

            students.update_status(student_id.trim().parse().unwrap(), status);
            // DEBUG: Print message after updating status
            println!("DEBUG: Student status updated successfully");
        } else if command == "graduate" {
            let mut student_id = String::new();

            println!("Enter student ID to graduate:");
            io::stdin()
                .read_line(&mut student_id)
                .expect("Failed to read line");

            students.graduate_student(student_id.trim().parse().unwrap());
            // DEBUG: Print message after graduating student
            println!("DEBUG: Student graduated successfully");
        } else if command == "exit" {
            println!("Goodbye!");
            break;
        } else {
            println!("Unknown command: {}", command);
            // DEBUG: Print message for unknown command
            println!("DEBUG: Unknown command received");
            // Don't break here, just continue the loop
        }
    }
}

fn take_command(cmd: &mut String) {
    println!("Enter command:");
    cmd.clear();
    io::stdin().read_line(cmd).expect("Failed to read line");
    *cmd = cmd.trim().to_string();
    // DEBUG: Print the command that was just read
    println!("DEBUG: take_command read '{}'", cmd);
}
