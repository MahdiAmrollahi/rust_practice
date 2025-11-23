// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
enum EmployeeType {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}
enum EmployeeStatus {
    Terminated,
    Active,
}
struct Employee {
    employee_type: EmployeeType,
    employee_status: EmployeeStatus,
}
fn can_access_building(employee: &Employee) -> Result<(), String> {
    match employee.employee_status {
        EmployeeStatus::Terminated => return Err("Employee is terminated".to_owned()),
        _ => (),
    }
    match employee.employee_type {
        EmployeeType::Maintenance => return Ok(()),
        EmployeeType::Marketing => return Ok(()),
        EmployeeType::Manager => return Ok(()),
        _ => return Err("Employee is not authorized to access the building".to_owned()),
    }
    Ok(())
}
fn print_access_result(employee: &Employee) -> Result<(), String> {
    can_access_building(employee)?;
    println!("Employee can access the building");
    Ok(())
}
fn main() {
    let employee = Employee {
        employee_type: EmployeeType::KitchenStaff,
        employee_status: EmployeeStatus::Terminated,
    };
    let result = print_access_result(&employee);
    match result {
        Err(e) => println!("Employee cannot access the building: {}", e),
        Ok(_) => (),
    }
}
