// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print


fn comp(a:i32)-> bool {
    let result= if a>100 {
        true
    } 
    else{
        false
    };
    result
}

fn printresult(a: bool) {
    
    match a {
        true => println!(">100"),
        false => println!("<=100"),

    }
}


fn main() {
    let x=100;
    printresult(comp(x));
}
