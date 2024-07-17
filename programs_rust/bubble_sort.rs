fn bubble_sort(v: &mut [i32]) {
    for i in 0..v.len() - 1 {
        let mut sorted = true;
        for j in 0..v.len() - 1 - i {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

fn main() {
    let mut v = vec![5, 3, 4, 2, 1];
    println!("{:?}", v);
    bubble_sort(&mut v);
    println!("{:?}", v);
    
    
}
