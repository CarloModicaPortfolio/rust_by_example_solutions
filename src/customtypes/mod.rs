pub mod rect_utils;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
pub struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    pub top_left: Point,
    pub bottom_right: Point,
}
