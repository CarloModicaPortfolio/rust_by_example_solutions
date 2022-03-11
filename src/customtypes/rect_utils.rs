use super::{Point, Rectangle};


pub fn rect_area(rect : Rectangle) -> f32{
    let base = rect.top_left.x -rect.bottom_right.x;
    let hight = rect.top_left.y -rect.bottom_right.y;
    return (base * hight).abs();
}
pub fn square(top_left: Point, side: f32) -> Rectangle{
    let bottom_right: Point = Point{x : top_left.x+side, y : top_left.y - side};
    let rect: Rectangle = Rectangle{top_left: top_left, bottom_right: bottom_right};
    return rect
}