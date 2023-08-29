// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Personal{
    age:i32,
    name:String,
    color:String,
}
fn print(a:&str){
    println!("{}",a );
}
fn main() {
    let name1=String::from("Alex");
    let name2=String::from("Jayson");
    let name3=String::from("Jeff");
    let color1=String::from("Blue");
    let color2=String::from("Green");
    let color3=String::from("Red");

    let persons = vec![Personal{age:24,name:name1,color:color1},
                        Personal{age:8,name:name2,color:color2},
                        Personal{age:5,name:name3,color:color3},];
    for person in persons{
        if person.age<=10 {
            print(&person.name);
            print(&person.color);
        }
    }
}
