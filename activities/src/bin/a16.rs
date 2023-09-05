// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Locker{
    name : String,
    assigned : Option<i32>,
}
fn main() {
    let student1 = Locker{
        name : "Rohan".to_owned(),
        assigned: Some(15),
    };
    match student1.assigned{
        Some(a) => println!("{} is assigned locker number: {}",student1.name,a ),
        None => println!(" {} is not assigned any locker",student1.name),
    }
}
