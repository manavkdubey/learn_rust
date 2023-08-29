// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery{
    Quantity: i32,
    Id : i32 
}

fn disp_quant(grocery : &Grocery){
    let a= grocery.Quantity;
    println!("{}",a );
}

fn disp_id(grocery : &Grocery){
    let a= grocery.Id;
    println!("{}",a );
}
fn main() {
    let grocery=Grocery{
        Quantity : 32,
        Id : 10
    };
    disp_quant(&grocery);
    disp_id(&grocery);
}
