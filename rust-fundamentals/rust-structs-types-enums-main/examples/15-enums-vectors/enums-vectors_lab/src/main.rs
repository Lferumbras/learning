/*
    Challenge Questions:

    Modify the code to add a new variant to the `Shape` enum, representing a Triangle shape. Update the `main` function to include a triangle shape in the `shapes` 
    vector and calculate the total area considering the triangle's dimensions.

    Extend the program by implementing a function that takes a vector of shapes as input and returns the largest shape based on its area. Invoke this function 
    with the `shapes` vector in the `main` function and print the details of the largest shape.
*/

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64,f64)
}

fn get_area(w: &Shape) -> f64 {
    match w {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Triangle(base,height) => (base * height)/2.0,
        _ => 0.0,
    }
}

fn get_largest_shape(s: &[Shape]) -> &Shape{
    let mut largest_shape = &Shape::Circle(0.0);
    for _s in s{
        if get_area(&_s) >= get_area(&largest_shape){
            largest_shape = _s;
        }
    }

    largest_shape
}

fn get_largest_shape_(s: &[Shape]) -> Option<&Shape> {
    s.iter().max_by(|a, b| {
        get_area(a)
            .partial_cmp(&get_area(b))
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}


fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(88.0,5.0)];

    println!("{}", get_area(&shapes[0]));

    let total_area: f64 = shapes
        .iter()
        .map(get_area)
        .sum();

    println!("Total area: {} sq. units", total_area);

    println!("The shape with largest area is {:?}", get_largest_shape(&shapes));

    if let Some(largest) = get_largest_shape_(&shapes) {
        println!("The shape with largest area is: {:?}", largest);
    }
}
