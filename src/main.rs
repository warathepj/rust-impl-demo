// Define a simple struct named 'Point'
struct Point {
    x: i32,
    y: i32,
}

// Use 'impl' to define methods for the 'Point' struct
impl Point {
    // An associated function (like a static method in other 
    // languages)
    // to create a new Point at the origin (0, 0).
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    // A method that calculates the distance from the origin 
    // for a Point instance.
    // '&self' indicates this method borrows the instance 
    // immutably.
    fn distance_from_origin(&self) -> f64 {
        let dx = (self.x - 0) as f64; // Calculate difference in x
        let dy = (self.y - 0) as f64; // Calculate difference in y
        (dx * dx + dy * dy).sqrt() // Calculate distance using Pythagorean theorem
    }
}

fn main() {
    // Create a Point instance using the associated function
    let p1 = Point::origin();

    // Create another Point instance
    let p2 = Point { x: 3, y: 4 };

    // Call the methods on the instances
    println!("Point 1 is at ({}, {})", p1.x, p1.y);
    println!("Point 2 is at ({}, {})", p2.x, p2.y);

    // Calculate and print the distance from the origin for p2
    println!("Distance from origin for Point 2: {}", p2.distance_from_origin());
}