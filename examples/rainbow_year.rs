use std::{error::Error, result::Result};
use tileline::{year_line, Config, DateDataSource, Datelike, Element, ElementLink, NaiveDate, Rgb};

struct YearDatasource {}
struct DateElement {
    date: NaiveDate,
}
impl Element for DateElement {
    fn get_color(&self) -> Rgb {
        match self.date.ordinal() {
            0..=50 => Rgb::new(255.0, 0.0, 0.0, None),
            51..=100 => Rgb::new(255.0, 127.0, 0.0, None),
            101..=150 => Rgb::new(255.0, 255.0, 0.0, None),
            151..=200 => Rgb::new(0.0, 255.0, 0.0, None),
            201..=250 => Rgb::new(0.0, 100.0, 255.0, None),
            251..=300 => Rgb::new(0.0, 0.0, 255.0, None),
            301.. => Rgb::new(128.0, 0.0, 255.0, None),
        }
    }

    fn get_border_color(&self) -> Rgb {
        match self.date.ordinal() {
            301.. => Rgb::new(255.0, 0.0, 0.0, None),
            251..=300 => Rgb::new(255.0, 127.0, 0.0, None),
            201..=250 => Rgb::new(255.0, 255.0, 0.0, None),
            151..=200 => Rgb::new(0.0, 255.0, 0.0, None),
            101..=150 => Rgb::new(0.0, 100.0, 255.0, None),
            51..=100 => Rgb::new(0.0, 0.0, 255.0, None),
            0..=50 => Rgb::new(128.0, 0.0, 255.0, None),
        }
    }

    fn get_link(&self) -> Option<Box<dyn ElementLink>> {
        None
    }
}
impl DateDataSource<DateElement> for YearDatasource {
    fn get_element(&self, data: chrono::NaiveDate) -> DateElement {
        DateElement { date: data.clone() }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new().build();
    let mut out = Vec::new();
    year_line(2023, YearDatasource {}, &mut out, config)?;
    std::fs::write("./target/rainbow_year.svg", &out).unwrap();
    Ok(())
}
