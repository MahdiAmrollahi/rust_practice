// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}
impl Person {
    fn new(age: i32, name: String, favorite_color: String) -> Self {
        Self { age, name, favorite_color }
    }
}
fn print_person(data: &str) {
    println!("{}", data);
}
fn main() {
    let people = vec![
        Person::new(10, "Ali".to_owned(), "Red".to_owned()),
        Person::new(20, "Reza".to_owned(), "Blue".to_owned()),
        Person::new(30, "Sina".to_owned(), "Green".to_owned()),
    ];
    for person in people {
        if person.age <= 10 {
            print_person(&person.name);
            print_person(&person.favorite_color);
        }
    }
}
