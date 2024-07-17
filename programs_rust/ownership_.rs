fn main() {
    let mut st = String::from("hellow ");
    takes_ownership(st.clone()); // if not clone then st will be passed to fake and st will no
                                 // longer be the owner of the (hellow data)
    println!(" st: {}", st);
    borrow_variable(&st);
    borrow_mut(&mut st);
    println!("st owner after borrower {}", st);
}
fn takes_ownership(fake: String) {
    println!("fake: {}", fake);
}
fn borrow_variable(s: &String) {
    println!("{} borrowed from st", s);
}
fn borrow_mut(s: &mut String) {
    s.push_str("world is bitcoin");
}
