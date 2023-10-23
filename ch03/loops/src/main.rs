fn main() {
    loop1();
    loop2();
    loop3();
    loop4();
    loop5();
    loop6();
}

fn loop1() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Loop 1 result is {result}");
    println!();
}

fn loop2() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    println!();
}

fn loop3() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!();
}

fn loop4() {
    // not the best way to do this, see loop5()
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);

        index += 1;
    }
    println!();
}

fn loop5() {
    // better way to do this
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {element}");
    }
    println!();
}

fn loop6() {
    // better way to do loop3()
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
