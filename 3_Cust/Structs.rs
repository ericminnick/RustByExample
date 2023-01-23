// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

//Added functions for RUSTEX activity
fn rect_area(rect: Rectangle) -> f32 {
    let points = rect;
    //let Point{x: top_x, y: top_y} = points.top_left;
    //let Point{x: bot_x, y: bot_y} = points.bottom_right;
   
    let Rectangle{top_left: Point{x: top_x, y: top_y},bottom_right: Point{x: bot_x, y: bot_y}} = points;

    let width = bot_x - top_x;
    let height = top_y - bot_y;
    width * height

}

//Added functions for RUSTEX activity
fn square(coordinate: Point, number: f32) -> Rectangle {
    let coord = coordinate;
    let side = number;
    let Point{x: top_x, y: top_y} = coord;
    let second_point = Point {
        x: (top_x + side),
        y: (top_y - side),
    };
    Rectangle{
    top_left: coord,
    bottom_right: second_point
    }
}
    

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, y: 1.0 };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    //ADDED Activity code
    println!("The area of the rectangle created by the 2 points is {:?}", rect_area(_rectangle));

    let sq_size = 3.0;
    println!("The area of a square created by Point {}, {} and a height/length of {} is {}", left_edge, top_edge, sq_size, rect_area(square(point, sq_size)));
    
}
