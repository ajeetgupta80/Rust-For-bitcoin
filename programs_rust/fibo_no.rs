use std::io;

fn fibo(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

fn main() {
    println!("type quit to exit");
    loop {
        let mut n = String::new();
        println!("enter number :");
        io::stdin().read_line(&mut n).expect("failed to read line");

        if n.trim() == "quit" {
            break;
        }
        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", fibo(n));
    }
}
