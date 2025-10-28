// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    Red,
    Green,
    Blue,
}
impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
        }
    }
}
struct Dimensions {
    length: i32,
    width: i32,
    height: i32,
}
impl Dimensions {
    fn print(&self) {
        println!("Length: {:?}", self.length);
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
    }
}
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}
impl Box {
    fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Self { dimensions, weight, color }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }

}
fn main() {
    let red_color = Color::Red;
    let dimensions = Dimensions { length: 11, width: 20, height: 30 };
    let small_box = Box::new(dimensions, 10.0, red_color);
    small_box.print();
}
