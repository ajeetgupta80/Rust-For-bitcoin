fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(30);
    for i in 4..100 {
        v.push(i);
    }
    println!("size of v: {:?}", v.len());
    println!("{:?}", v);
}
