// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Person<'a> {
    name: &'a str,
    title: &'a str,
}

fn main() {
    let mut persons: Vec<Person> = Vec::new();

    for (i, line) in MOCK_DATA.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let fields: Vec<&str> = line.split(',').collect();
        // id,first_name,email,dept,title
        // 0  1          2     3    4
        if fields.len() >= 5 {
            persons.push(Person {
                name: fields[1],
                title: fields[4],
            });
        }
    }

    for person in persons {
        println!("Name: {}, Title: {}", person.name, person.title);
    }
}
