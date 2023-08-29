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

enum BoxColor{
    Blue,
    Red,
    Green,
}
struct Box{
    Length:i32,
    Breadth:i32,
    Height:i32,
}

impl Box{
    fn entry()->Self{
        Self{
            Length:100,
            Breadth:200,
            Height:300
        }
        

    }
    fn show_dim(&self,color:BoxColor){
        match color{
            BoxColor::Blue => println!("Blue"),
            BoxColor::Red => println!("Red"),
            BoxColor::Green => println!("Green"),
        }
        println!("Length:{} Breadth:{} Height:{}",self.Length,self.Breadth,self.Height );
    }
}
fn main() {
    let box1 =Box{
        Length:10,
        Breadth:20,
        Height:30
    };
    let box1color=BoxColor::Blue;

    Box::show_dim(&box1,box1color);
    let new = Box::entry();
    new.show_dim(BoxColor::Red);
}
