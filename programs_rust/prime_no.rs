fn checkprime(num: i32, i: i32) -> i32 {
    if (i == 1) {
        return 1;
    } else {
        if (num % i == 0) {
            return 0;
        } else {
            return checkprime(num, i - 1);
        }
    }
}

fn main() {
    let num: i32 = 11;
    let rs = checkprime(num, num / 2);

    if (rs == 1) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}
