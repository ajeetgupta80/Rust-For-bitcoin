fn main() {
    let mut something: String = String::from("nothing sexier than bitcoin");
    println!("{}", something);

    for i in 0..10 {
        something = something + " hey";
        println!("{}", something);
    }
    let word = get_first_word(something);
    println!("{}", word);
}

fn get_first_word(word: String) -> String {
    let mut ans = String::from("");
    for char in word.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

