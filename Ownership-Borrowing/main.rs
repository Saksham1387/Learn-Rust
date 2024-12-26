// ================================================
// 🦀 Rust Basics: Learn Ownership and Bowrrowing 🦀
// ------------------------------------------------
// This file covers:
// - 📦 structs (immutable & mutable)
// - 🔢 Enums
// - 🔄 Pattern Matching
// - 🔁 Ownership
// - 📜 Borrowing and References
// ------------------------------------------------
// Follow along to master Ownership in Rust
// ================================================

fn main() {
    // ===== Structs =====
    // Structs are like Objects in Typescript
    // They allows us to structure and store similar data together
    struct User {
        name:String;
        age:u32;
        active:bool;
    }

    // This is how you define a struct to use it
    let user1 = User{
        name:"saksham",
        age:"21",
        active:false
    };

    // This is how you access the data of a struct
    print!("The user named {} is {} years old",user1.name,user1.age)

    // You can also implement a struct i.e make a function over it, that you can execute
    struct Rect {
        width: u32;
        height: u32
    }

    impl Rect{
        // Here self is passed by reference
        fn area(&self)->u32{
            return self.width * self.height;
        }
        print!("The area of the reactangle is {}", rect.area())
    }


    // ===== Enums =====
    enum Direction{
        North,
        South,
        East,
        West
    }
    let direction = Direction::North;
    move_player(direction);

    fn move_player(direction:Direction){
        // You can also use match to match on enums
        // Here you are simply mathcing on the enums if the direction is north or south or east or west
        match direction{
            Direction::North => print!("Going North"),
            Direction::South => print!("Going South"),
            Direction::East => print!("Going East"),
            Direction::West => print!("Going West"),
        }
    }


    // ===== Pattern Matching =====
    // The enums can also contain data
    enum Shape {
        Square(u32),
        Rectangle(u32,u32),
    }

    let shape = Shape::Rectangle(10,20);

    fn draw_shape(shape:Shape){
        match shape{
            Shape::Square(size) => print!("The size of the square is {}",size),
            Shape::Rectangle(width,height) => print!("The size of the rectangle is {} x {}",width,height),
        }
    };

    let res = fs::read_to_string("exampltxt");
    match rse {
        Ok(content) => {
            println("{}",content)
        }

        Err(error) => {
            println("{}",error)
        } 
    }


    // ===== Ownership =====
    // In rust to avoid error like a dangling pointer error 
    // the complier itself forces you to write the code in such a way that there is no possiblity to write anything that would
    // cause such error

    // Here a single value can have only one owner i.e 
    let mut a = 23;
    // here a owns 23
    let b = a
    println("{}",a)
    // Now in other languages the above print statement would work but here it won't 
    // Now the a does not have any value the "23" is transferred to 'b' so you cannot print 'a' here

    // But now you can print a becuase you transferred the ownership back to 'a'
    a = b
    println("{}",a) // This will work


    // ===== Borrowing and References =====
    // Now whatif you really want to use a value without changing the owner 
    // This is how you do it
    let c = 34;
    let d = &c

    // Here you can still print the c variable
    // As you passed it in refence meaning you borrows the value and did something with it and then returned back to it
    println("{}",c)

    // Similarly when you pass a value to a function, it also takes away the ownership of a value
    // For example
    let x = 23
    sqaure_root(x)

    fn sqaure_root(x:u32) -> u32{
        return x ** x
    }

    // Now again you cannot print the value of x because that is still not the owner of 23 as it has pased the ownership to the 
    // sqaure_root() function 

}
