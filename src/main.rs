use std::fs::OpenOptions;
use tileline::{tile, Config, Element, Rgb, ElementLink};

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
    fn get_link(&self) -> Option<Box<dyn ElementLink>> {
        Some(Box::new(ElementLinkImpl("https://tglman.com".to_owned(), "Tglman.com".to_owned())))
    }
}
struct ElementLinkImpl(String,String);
impl ElementLink for ElementLinkImpl {
    fn link(&self) ->String {
        self.0.clone()
    }
    fn title(&self) ->String {
        self.1.clone()
    }
}

fn main() {
    let mut val = Vec::new();
    for i in 0..5 {
        let mut column = Vec::new();
        for z in 0..30 {
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
