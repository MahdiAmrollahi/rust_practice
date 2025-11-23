use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        if let Some(bill) = self.inner.get_mut(name) {
            bill.amount = amount;
            true
        } else {
            false
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added");
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        if bills.remove(&name) {
            println!("bill removed");
        } else {
            println!("bill not found");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to update:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        if bills.update(&name, amount) {
            println!("bill updated");
        } else {
            println!("bill not found");
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBills,
    RemoveBill,
    UpdateBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBills),
            "3" => Some(MainMenu::RemoveBill),
            "4" => Some(MainMenu::UpdateBill),
            _ => None,
        }
    }
    fn show() {
        println!("");
        println!("== Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("");
        println!("Enter selection: ");
    }
}

fn main() {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_input().expect("no data entered");
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBills) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => break,
        }
    }
}
