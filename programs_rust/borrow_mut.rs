fn main() {
    let mut st = String::from("hello");
    let k = &mut st;

    k.push_str("world");
    println!("k {}", k);
    println!("st {}", st);
    something_weired();
}

fn something_weired() {
    let mut name = String::from("ajeet");
    let y = &mut name;
    y.push_str("gupta");
    println!("updated name {}", y);
    let a = &name;
    let b = &name;
    let c = &name;
    println!("a {}", a);
}
