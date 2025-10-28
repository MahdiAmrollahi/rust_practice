// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavor {
    Sparkling,
    Sweet,
    Sour,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    ounces: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Sparkling"),
        Flavor::Sweet => println!("Sweet"),
        Flavor::Sour => println!("Sour"),
        Flavor::Fruity => println!("Fruity"),
    }
    println!("{} ounces", drink.ounces);
}
fn main() {
    let drink = Drink {
        flavor: Flavor::Sparkling,
        ounces: 12.0,
    };
    print_drink(drink);
    let drink = Drink {
        flavor: Flavor::Sweet,
        ounces: 10.0,
    };
    print_drink(drink);
    let drink = Drink {
        flavor: Flavor::Sour,
        ounces: 8.0,
    };
    print_drink(drink);
}
