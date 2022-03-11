pub mod formatted_print;
use std::fmt; // Import `fmt`


// COMPLEX DISPLAY
// ================================================================================================
#[derive(Debug)]
pub(crate) struct Complex {
    pub(crate) real: f64,
    pub(crate) imag: f64,
}
// Implement `Display` for 'Complex'.
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        if self.imag > 0.0 {
            write!(f, "{}+{}i", self.real, self.imag)
        }
        else{
            write!(f, "{}-{}i", self.real, self.imag.abs())
        }
    }
}

// VECTOR INDEX ELEMENT DISPLAY
// ================================================================================================

// Define a structure named `List` containing a `Vec`.
#[derive(Debug)]
pub(crate) struct List(pub Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: ", count)?;
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}


// COLOR DISPLAY
// ================================================================================================
#[derive(Debug)]
pub struct Color {
    pub  red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for Color {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        // RGB (128, 255, 90) 0x80FF5A
        // format!("0x{:X}", foo) -> "0xDEADBEEF"
        let r = format!("{:X}", self.red);
        let g = format!("{:X}", self.green);
        let b = format!("{:X}", self.blue);
        write!(f, "RGB ({}, {}, {}) 0x{:0>2}{:0>2}{:0>2}",
               self.red, self.green, self.blue, r, g, b)
    }
}