# Rust impl Demo

A simple Rust project demonstrating the use of `impl` blocks for defining methods on structs.

## Overview

This project showcases how to:

- Define a struct (`Point`) with fields
- Implement methods for the struct using `impl` blocks
- Create associated functions (similar to static methods in other languages)
- Use instance methods that operate on struct instances

## Code Explanation

The project defines a `Point` struct representing a 2D point with x and y coordinates:

```rust
struct Point {
    x: i32,
    y: i32,
}
```

Methods are implemented for the Point struct using an impl block:

```rust
impl Point {
    // Associated function (static method)
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    // Instance method
    fn distance_from_origin(&self) -> f64 {
        let dx = (self.x - 0) as f64;
        let dy = (self.y - 0) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}
```

## Running the Project

To run this project:

```bash
cargo run
```

Expected output:

```bash
Point 1 is at (0, 0)
Point 2 is at (3, 4)
Distance from origin for Point 2: 5
```

## Key Concepts

1. Struct Definition : Defines a custom data type with named fields
2. impl Blocks : Groups methods and associated functions for a type
3. Associated Functions : Called with Type::function() syntax (no instance needed)
4. Instance Methods : Take &self or &mut self as first parameter to operate on instances
