fn main() {
    /*
        Ownership
        Ownership describes a set of rules that govern how rust deals w/ memory. In principle there
        are three main rules:
            - Each value has an owner.
            - There can only be one owner at a time.
            - When the owner goes out of scope, the value will be dropped.
    */

    // The String type is perfect for demonstration purposes because it's allocated at the heap and not in the stack
    // like any string literal. And that occurs because strings of type str are immutable w/ a known size at compile time, the opposite of
    // variables of type String, w/ unkown size at compile time.
    let str = String::from("Hello, world!");
    let str2 = str;

    println!("str is {}", str);

    take_ownership(str);

    // The following line cannot leave the program to compilate.
    // println!("String is {}", str);

    // And that occurs because the "str" was moved into the "take_ownership" function scope. It cannot be used anymore because the "take_ownership" function
    // has getted the ownership of this variable. This is called as a "move". This concept is applied to data that is stored at the heap and not in the stack.

    // Well, what if it's neccessary to still using this variable? 
    // A way would be returning the value in the function, but this is not elegant.
    // Instead, we could change the function signature to use a "reference" to that data, and not the data itself.
    // So, the following code is valid:

    let str: String = String::from("Another string.");
    use_but_not_take_ownership(&str);

    println!("String is {}", str);

    // The above code shows another concept of ownership: borrowing. In this context,
    // the string "str" was borrowed, and not moved or copied.
}

fn take_ownership(str: String) {
    println!("String is {}", str);
}

fn use_but_not_take_ownership(str: &String) {
    println!("String is {}", str);
}
