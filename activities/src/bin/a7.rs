// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum colour{
    Violet,
    Indigo,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
}

fn which_color(entry: colour){
    match entry{
        colour::Violet=>println!("violet"),
        colour::Indigo=>println!("indigo"),
        colour::Blue=>println!("blue"),
        colour::Green=>println!("green"),
        colour::Yellow=>println!("yellow"),
        colour::Orange=>println!("orange"),
        colour::Red=>println!("red"),
    }
}

fn main() {
    which_color(colour::Violet)
}

