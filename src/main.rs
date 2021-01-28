include!("single.rs");
include!("grid.rs");
extern crate minifb;
extern crate image;

use minifb::{Key, Window, WindowOptions};
use std::path::Path;
use std::fs::File;


fn main() {
    grid();
}
natural_keys