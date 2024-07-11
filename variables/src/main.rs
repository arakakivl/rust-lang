fn main() {
    /*
        Variables, by default, are immutable. Thus, its value or its type can't be changed if they're not
        mutable. For example, the following commented piece of code can't allow the code to be compiled
        because an attempt of changing an immutable variable occurred.
    */

    // let i: u8 = 2;
    // i += 1;

    /* But the following code is allowed. */
    let mut a: u8 = 2;
    println!("i is {a}");

    a += 1;
    println!("Now, i is {a}.");

    /*
        There are constants too, and they differ from immutable variables. For example, by shadowing them, immutable
        variables can be mutable ones, but constants cannot. Furthermore, they need to have their type explicity annotated,
        and its assigned value must be a constant expression -- and not a computed runtime result. They're declared with the
        "const" keyword.
        ** Constants can be declared in any scope, and it's a convention to name them in THIS_CASE.
    */

    const B: u8 = 10;
    println!("B is {B}");   

    // const B: u8 = 40; Shadowing it isn't allowed for constants variables!

    /*
        Shadowing makes a variable being declared again, leaving all the responsability of that variable to the new one.
        Its type, mutability and value can be changed.
    */

    let c = 20;
    println!("c is {c}.");

    let c = c + c;
    println!("Now, c = c + c ==> {c}");

    {
        let c = c + c + c;
        println!("In this scope, c = c + c + c ==> {c}");
    }

    println!("But out of the scope, c still the same value {c}.");
}
