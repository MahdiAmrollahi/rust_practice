// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
enum Status {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rental {
    vehicle_type: String,
    vin: String,
    status: Status,
}

struct Corporate {
    rentals: Rc<RefCell<Vec<Rental>>>,
}

struct StoreFront {
    rentals: Rc<RefCell<Vec<Rental>>>,
}

fn main() {
    let rentals = vec![
        Rental {
            vehicle_type: "Sedan".to_string(),
            vin: "123".to_string(),
            status: Status::Available,
        },
        Rental {
            vehicle_type: "SUV".to_string(),
            vin: "456".to_string(),
            status: Status::Available,
        },
    ];

    let rentals = Rc::new(RefCell::new(rentals));

    let corporate = Corporate {
        rentals: Rc::clone(&rentals),
    };

    let storefront = StoreFront {
        rentals: Rc::clone(&rentals),
    };

    // Change status from StoreFront
    {
        let mut rentals = storefront.rentals.borrow_mut();
        if let Some(car) = rentals.iter_mut().find(|r| r.vin == "123") {
            car.status = Status::Rented;
            println!("StoreFront rented out car 123");
        }
    }

    // Change status from Corporate
    {
        let mut rentals = corporate.rentals.borrow_mut();
        if let Some(car) = rentals.iter_mut().find(|r| r.vin == "456") {
            car.status = Status::Maintenance;
            println!("Corporate put car 456 into maintenance");
        }
    }

    // Verify changes
    println!("Final Rental Status:");
    let rentals = corporate.rentals.borrow();
    for rental in rentals.iter() {
        println!("{:?}", rental);
    }
}