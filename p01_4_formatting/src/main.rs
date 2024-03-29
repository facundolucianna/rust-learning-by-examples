/* Activity:
* Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:
* RGB (128, 255, 90) 0x80FF5A
* RGB (0, 3, 254) 0x0003FE
* RGB (0, 0, 0) 0x000000
*/

use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        // Both works!
        // Solution #1
        let rgb = self.red as u32 * 0x10000 + self.green as u32 * 0x100 + self.blue as u32;
        write!(f, "RGB ({}, {}, {}) 0x{:0<6X}", self.red, self.green, self.blue, rgb)

        // Solution #2
        //write!(f, "RGB ({red}, {green}, {blue}) 0x{red:0<2X}{green:0<2X}{blue:0<2X}",
        //       red=self.red,
        //       green=self.green,
        //       blue=self.blue)
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        println!("{}", color);
    }
}
