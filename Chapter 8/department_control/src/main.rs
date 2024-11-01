use std::collections::HashMap;

fn main() {
    let mut office: HashMap<String, Vec<String>> = HashMap::new();
    let mut departments: Vec<String> = Vec::new();

    show_commands();

    loop {   
        let mut buf = String::new();

        std::io::stdin().read_line(&mut buf).expect("Couldn't read line");
        let temp = buf.trim().to_uppercase();
        let mut buf = temp.split_whitespace();

        let amount = buf.clone().count();

        if let Some(command) = buf.next() {
            match command {
                "ADD" => {
                    if amount == 4 {
                        let employee_name = buf.next().unwrap().to_string();
                        if buf.next().unwrap().to_string() != "TO" {
                            println!("Incorrect usage of add command\n");
                            continue;
                        }
                        let department = buf.next().unwrap().to_string();
                        add(&mut office, employee_name, department, &mut departments);
                    }
                    else {
                        println!("Incorrect usage of add command\n");
                    }
                },
                "PRINT" => {
                    if amount == 2 {
                        people_department(&office, buf.next(), &mut departments)
                    }
                    else {
                        println!("Incorrect usage of print command\n")
                    }
                }
                "VIEW" => show_commands(),
                "EXIT" => {
                    println!("Exiting application\n");
                    return;
                }
                _ => println!("Command does not exist\n"),
            }
    }   }
}

fn add(office: &mut HashMap<String, Vec<String>>, employee_name: String, department: String, departments: &mut Vec<String>) {
    let entry = office.entry(department.clone()).or_insert(Vec::new());
    entry.push(employee_name.clone());
    entry.sort();
    println!("\nAdded {} to {} department", employee_name, department);
    if !departments.contains(&department) {
        departments.push(department);
        departments.sort();
    }
}

fn people_department(office: &HashMap<String, Vec<String>>, result: Option<&str>, departments: &mut Vec<String>) {
    let department;
    if let Some(value) = result {
        if value == "ALL" {
            all_people(office, departments);
            return;
        }
        department = value.to_string();
    }
    else {
        println!("Incorrect usage of print command\n");
        return;
    }

    let vector = match office.get(&department) {
        Some(vector) => vector,
        None => {
            println!("There is no department that matches the given name\n");
            return;
        }
    };

    println!("\nEmployees at the {} department:\n", department);
    for person in vector {
        println!("- {}", person);
    }
}

fn all_people(office: &HashMap<String, Vec<String>>, departments: &mut Vec<String>) {
    for department in departments {
        println!("\nEmployees at the {} department:\n", department);
        let vector = office.get(department).unwrap();
        for person in vector {
            println!("- {}", person);
        }
    }
}

fn show_commands() {
    println!("\nCommands:\n");
    println!("Add - Adds an employee to a department                              \t[Usage]: Add Employee_Name to Department");
    println!("Print - Prints the people on the specified department on the screen \t[Usage]: Print Department (Use keyword 'all' for all departments)");
    println!("View - Shows this screen");
    println!("Exit - Exits the program\n");
}