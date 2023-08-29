// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket{
    Backstage(String,f64),
    Vip(String,f64),
    Standard(f64),

}


fn main() {
    
    let mut tickets= Vec::new();
    tickets.push(Ticket::Backstage("John".to_owned(),500.00));
    tickets.push(Ticket::Vip("Jeff".to_owned(),1000.00));
    tickets.push(Ticket::Standard(200.00));

    for tic in tickets{
        match tic{
            Ticket::Backstage(name,price)=>println!("Backstage access Name:{},Ticket price:{}",name,price),
            Ticket::Vip(name,price)=>println!("Vip access Name:{},Ticket price:{}",name,price),
            Ticket::Standard(price)=>println!("Standard access Ticket price:{}",price),
            
        }
    }

}
