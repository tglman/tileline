use std::fs::File;
use tileline::{tile, Config, Element, ElementLink, Rgb};

struct Value(Rgb, Rgb);

impl Value {
    fn new(background: &str, border: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(Rgb::from_hex_str(background)?, Rgb::from_hex_str(border)?))
    }
}

impl Element for Value {
    fn get_color(&self) -> Rgb {
        self.0.clone()
    }

    fn get_border_color(&self) -> Rgb {
        self.1.clone()
    }

    fn get_link(&self) -> Option<Box<dyn ElementLink>> {
        None
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut val = Vec::new();
    val.push(
        vec![
            Value::new("#009ECE", "#644436")?,
            Value::new("#FF9E00", "#218559")?,
            Value::new("#F7D708", "#4EB5D6")?,
        ]
        .into_iter(),
    );
    val.push(
        vec![
            Value::new("#aaaaaa", "#274257")?,
            Value::new("#CE0000", "#2A75A9")?,
            Value::new("#9CCF31", "#8F6048")?,
        ]
        .into_iter(),
    );

    let config = Config::new().offset_x(10).offset_y(10).build();
    let mut out = File::create("./target/logo.svg")?;
    tile(config.clone(), val.into_iter(), &mut out).unwrap();
    Ok(())
}
