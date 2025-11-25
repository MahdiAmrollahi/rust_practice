// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
    let tiles = vec![
        Tile::Brick(BrickStyle::Red),
        Tile::Brick(BrickStyle::Dungeon),
        Tile::Water(Pressure(15)),
        Tile::Water(Pressure(5)),
        Tile::Grass,
        Tile::Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount: 200,
        }),
        Tile::Treasure(TreasureChest {
            content: TreasureItem::SuperPower,
            amount: 200,
        }),
        Tile::Wood,
    ];

    for tile in tiles {
        match tile {
            Tile::Brick(brick @ BrickStyle::Gray) | Tile::Brick(brick @ BrickStyle::Red) => {
                println!("The brick color is {:?}", brick)
            }
            Tile::Brick(brick) => println!("{:?} brick", brick),
            Tile::Water(p) if p.0 >= 10 => println!("High water pressure!"),
            Tile::Water(p) => println!("Water pressure level: {:?}", p.0),
            Tile::Grass | Tile::Dirt | Tile::Sand => println!("Ground tile"),
            Tile::Treasure(TreasureChest {
                content: TreasureItem::Gold,
                amount,
            }) if amount >= 100 => println!("Lots of gold!"),
            _ => (),
        }
    }
}
