use std::collections::HashMap;

fn main() {
    let mut bitcoin_holders: HashMap<String, u32> = HashMap::<String, u32>::new();
    bitcoin_holders.insert("ajeet".to_string(), 400);
    bitcoin_holders.insert("abhay".to_string(), 400);
    bitcoin_holders.insert("arpit".to_string(), 400);
    bitcoin_holders.insert("abhishek".to_string(), 400);
    println!("{:#?}", bitcoin_holders);
    for (x, y) in bitcoin_holders {
        println!("{} holds {} bitcoins", x, y);
    }

}
