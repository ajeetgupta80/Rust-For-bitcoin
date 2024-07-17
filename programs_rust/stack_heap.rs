fn main() {
    stack_();
    heap_();
    update_heap_();
}

fn stack_() {
    let a = 10;
    let b = 20;
    let c = 30;
    println!("a: {}, b: {}, c:{}", a, b, c);
}

fn heap_() {
    let x = String::from("learning");
    let y = String::from("bitcoin");
    let z = format!("{},{}", x, y);
    println!("{}", z);
}
fn update_heap_() {
    let mut st = String::from("learning bitcoin is not my cup of tea");

    for _ in 0..500 {
        st.push_str("is hard and harder ");
        println!(
            "cap {}, len {}, pointer {:p}",
            st.capacity(),
            st.len(),
            st.as_ptr()
        );
    }
}
