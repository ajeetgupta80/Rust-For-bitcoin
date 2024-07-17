use std::fs;
fn main() {
    let res = fs::read_to_string("nothing.txt");

    match res {
        Ok(content) => {
            println!("file content: {}", content);
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }
    println!("bitcoin is hard but also harder");
}

