fn main() {
    /* 
        There are scalar and compound types.
        A scalar type variable represents a single value, and there are four primitive types:
        integers, floats, chars and booleans.
        Instead, compound types group multiple values into a single type. There are two primitive of them:
        tuples and arrays.
    */

    // Signed integer of 8 bits:
    let a: i8 = 127;
    println!("a is {a}");

    // Unsigned integer of 8 bits:
    let a: u8 = 255;
    println!("a is {a}");

    // Floating-point of 32 bits:
    let a: f32 = 1.14;
    println!("a is {a}");

    // Floating-point of 64 bits:
    let a: f64 = 1.03214214;
    println!("a is {a}");

    // Character "char" type (four bytes in size, being unicode encoded):
    let a: char = 'A';
    println!("a is {a}");

    // Boolean type:
    let a: bool = false;
    println!("a is {a}");

    /*
        Tuple is a type that groups any number of values of different types together.
        They cannot be shrinked or changed. The following syntax declares a tuple:
    */

    let a: (i32, f32, char) = (200, 1.2, 'A');
    
    // Destructuring tuple
    let (x, y, z) = a;
    println!("x is {x}, y is {y} and z is {z}");
    
    // Accessing tuple data
    let a = a.0;
    println!("a is {a}.");

    /*
        An array is a collection of values of the same type. They have a fixed length and all
        data stored inside them is allocated on the stack rather than heap.
    */

    // Declaring an array of strings
    let _a = [ "jan", "feb", "mar" ];

    // Explicity indicating the array length and its value types.
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // Creating an array with seven elements which all of them have the same value.
    let a = [5; 7];
    let second_index = a[2];

    // Accessing an element.
    println!("Index two is {second_index}")

}
