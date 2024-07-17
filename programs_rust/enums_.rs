enum Shape {
    Circle(u32),
    Square(u32),
    Rectangle(u32, u32),
}
fn calculate_area(shape: Shape) -> u32 {
    let ans = match shape {
        Shape::Circle(radius) => 0,
        Shape::Rectangle(height, base) => height * base,
        Shape::Square(side) => side * side,
    };
    return ans;
}

fn main() {
    let circ = Shape::Circle(10);
    let square = Shape::Square(20);
    let rect = Shape::Rectangle(40, 50);

    println!("area {}", calculate_area(rect));
}
