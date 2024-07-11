fn main() {
    /*
        Statments do not return a value; but expressions do. 
    */

    // This is a statment.
    let _a = 5;

    // This is an expression, like any function, macro or created scopes with curly brackets.
    // Expressions don't end in semicolons. Adding a semicolon into the end of an expression
    // turn it into a statment.
    let y = {
        let x = 3;
        x + 2 // ;
    };

    println!("y is {y}");

    // Passing parameters and getting function return value.
    let y = sum(y, y);
    println!("y is {y}");

    do_something();
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn do_something() {
    println!("Something!");
    return;

    // println!("This won't execute.");
}