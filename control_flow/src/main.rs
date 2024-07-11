fn main() {
    // If expressions
    let mut n = 5;
    if n <= 5 {
        println!("n <= 5");
    } else {
        println!("n > 5");
    }

    n = 6;

    // Else if expressions
    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 4, 3, or 2");
    }

    // Using if expressions inside a let statment (it's like a ternary operator).
    n = if n < 6 { 6 } else { 7 };
    println!("n is {n}");

    // loop expression
    n = 0;
    loop {
        println!("This would run forever but a break expression is encountered when n == 5!");
        n += 1;
        
        if n == 5 { break; }
    }

    // Because loop is an expression, it can returns value too.
    n = loop {
        n += 1;
        if n > 15 { break n * 2; }
    };

    println!("n is {n}");

    n = 0;
    let mut i = 0;

    n = loop { 
        i += 2;
        if i >= 15 { break i; }
    };

    println!("n is {n}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count is {count}.");
        let mut remaining = 10;

        'inner_loop: loop {
            println!("remaining is {remaining}.");

            if remaining == 9 {
                break 'inner_loop
            } else if count == 2 { 
                break 'counting_up 
            }

            remaining -= 1;
        }

        count += 1;
    }

    // Conditional loop with while
    n = 0;
    while n < 1000000 {
        n += 423;
    }

    println!("n is {n}");

    // looping through a collection like an array
    let a = [ 10, 20, 30, 40, 50 ];
    let mut i = 0;

    for e in a {
        println!("a[{i}] = {e}");
        i += 1;
    }

    // Using Range provided by the standard library.
    for e in 1..50 {
        println!("{e}");
    }

}
