mod helloworld;
mod primitives;
mod customtypes;

use helloworld::{formatted_print, Complex, List, Color};
use formatted_print::formattedprints;
use primitives::{Matrix, matrix_utils};
use matrix_utils::transpose;
use customtypes::{Point, Rectangle,rect_utils::rect_area, rect_utils::square};


fn main() {

    println!("Hello, world! Carlo is ready to rust");

// EX. 1
// ===============================================================================================
    println!("EX. 1");
    formattedprints();
    println!();


// EX. 2
// ================================================================================================
    println!("EX. 2");
    let complex = Complex{real:-5.2, imag: 8.7};
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
    println!();



// EX. 3
// ================================================================================================
    println!("EX. 3");
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    println!();

// EX. 4
// ================================================================================================
    println!("EX. 4");
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }
    println!();

// EX. 5
// ================================================================================================
    println!("EX. 5");
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // println!("{}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
    println!();

// EX. 6
// ================================================================================================
    println!("EX. 6");
    let top_l: Point = Point { x: 10.0, y: 10.0 };
    let bot_r: Point = Point { x: 0.0, y: 0.0 };

    let rect_1: Rectangle = Rectangle{top_left : top_l, bottom_right : bot_r };
    println!("Rectangle area is: {}", rect_area(rect_1));
    let top_l_2: Point = Point { x: 4.0, y: 4.0 };
    let rec_2: Rectangle = square(top_l_2, 5.0);
    println!("New rect: {:?}", rec_2);

    println!();
}
