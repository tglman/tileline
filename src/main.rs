use std::fs::OpenOptions;
use tileline::{tile, Config, Element, Rgb};

struct Value {
    i: u16,
}
impl Value {
    fn new(i: u16) -> Self {
        Self { i }
    }
}
impl Element for Value {
    fn get_color(&self) -> Rgb {
        Rgb::new(0.0, 0.0, (self.i + 100) as f64, None)
    }
    fn get_border_color(&self) -> Rgb {
        Rgb::new(0.0, 0.0, 50.0, None)
    }
}

fn main() {
    let mut val = Vec::new();
    for i in 1..5 {
        let mut column = Vec::new();
        for z in 1..30 {
            column.push(Value::new(i * 10 + z * 10));
        }
        val.push(column.into_iter());
    }
    let config = Config::new().build();

    let f = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("output.svg")
        .unwrap();
    tile(config, val.into_iter(), f).unwrap();
}
