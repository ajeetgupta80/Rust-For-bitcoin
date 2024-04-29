use std::io;

fn factorial_recursive(x: i32) -> i32 {
    if x <= 1 {
        return 1;
    }

    return x * factorial_recursive(x - 1);
}
fn main() {
    // let mut n = String::new();
    // io::stdin().read_line(&mut n).expect("error");

    //let x: i32 = n.trim().parse().expect("its just a overhead");
    // println!("integer value {}", x);
    // println!("string {}", n);
    // let mut x = 10;
    // println!("value of x is {}", x);
    // x = 9;
    // println!("value of changed x {}", x);

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("something wrong");
    let x: i32 = n.trim().parse().expect("something wrong");

    let factorial = factorial_recursive(x);
    println!("factorial of x is {}", factorial);
}
