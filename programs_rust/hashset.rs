use std::collections::HashSet;

fn main() {
    let mut bitcoin_hodl = HashSet::<String>::new();
    let mut number = HashSet::<i32>::new();
    bitcoin_hodl.insert("ajeet".to_string());
    bitcoin_hodl.insert("ajeet".to_string());
    bitcoin_hodl.insert("abhay".to_string());
    bitcoin_hodl.insert("abhay".to_string());
    bitcoin_hodl.insert("arpit".to_string());

    number.insert(10);
    number.insert(10);
    number.insert(30);
    println!("{:?}", number);

    println!("{}", bitcoin_hodl.len());
    println!("{:?}", bitcoin_hodl);
}
