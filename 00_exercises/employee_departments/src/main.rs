use std::io;
use std::io::Write;
use std::collections::HashMap;

// -------------------------- STRUCTS & ENUMS

enum CMD  {
  Add,
  List,
  Exit,
  Unknown
}

struct Departments {
  name: String,
  employees: Vec<String>,
}

impl Departments {
  fn add_employee(&mut self) {
    print!("Name of the employee: ");

    let mut employee = line_reader();

    self.employees.push(employee.trim().to_string());
  }

  fn list_employees(&self) {
    for employee in self.employees.iter() {
      println!("{} works in {}", employee, self.name);
    }
  }
}

// -------------------------- HELPERS

fn line_reader() -> String {
  io::stdout().flush().ok().expect("Could not flush stdout");

  let mut input = String::new();

  io::stdin().read_line(&mut input).expect("Failed to read line");

  input
}

fn choose_department() {
  print!("For department: ");

  let mut department = line_reader();
}

// -------------------------- MAIN

fn main() {
  let mut sales = Departments { name: "Sales".to_string(), employees: Vec::new() };
  let mut engineering = Departments { name: "Engineering".to_string(), employees: Vec::new() };

  loop {
    let mut cmd = String::new();

    io::stdin().read_line(&mut cmd).expect("Failed to read line");

    let cmd = cmd.trim().to_lowercase();

    // it can also be cmd.as_ref()
    match &cmd[..] {
      "add" => engineering.add_employee(),
      "list" => engineering.list_employees(),
      "exit" => break,
      _ => println!("unknown cmd => {:?}", cmd),
    }
  }
}
