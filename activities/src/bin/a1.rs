// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name()->&'static str{
    let my_first_name="Manav";
    my_first_name
}

fn last_name()->&'static str{
    let my_last_name="Kumar Dubey";
    my_last_name
}


fn main(){
    println!("{} {}",first_name(),last_name());

}
