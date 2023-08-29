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

enum Flavor{
    Orange,
    Apple,
    Choclate,
    Cola,
}

struct Drink{
    flavoring: Flavor,
    fluid_ounce: i32,
}
fn flovourprint(drink:Drink){
    match drink.flavoring{
        Flavor::Orange=>println!("Orange"),
        Flavor::Apple=>println!("Apple"),
        Flavor::Choclate=>println!("Choclate"),
        Flavor::Cola=>println!("Cola"),
    }
    println!("{}",drink.fluid_ounce );
}

fn main() {
    let my_drink=Drink{
        flavoring : Flavor::Orange,
        fluid_ounce : 31,
    };
    flovourprint(my_drink);
    
}
