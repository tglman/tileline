use builder_pattern::Builder;
pub use colorsys::Rgb;
use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::Writer;
use std::{io::Write, sync::Mutex};

#[derive(Builder)]
pub struct Config {
    #[default(0)]
    offset_x: u32,
    #[default(0)]
    offset_y: u32,
    #[default(15)]
    size_x: u32,
    #[default(15)]
    size_y: u32,
    #[default(10)]
    padding_x: u32,
    #[default(10)]
    padding_y: u32,
    #[default(1)]
    border: u32,
}

fn coordinate(base: u32, size: u32, padding: u32, offset: u32) -> u32 {
    offset + (base * (size + padding))
}
impl Config {
    fn size_x(&self) -> String {
        format!("{}", self.size_x)
    }
    fn size_y(&self) -> String {
        format!("{}", self.size_x)
    }
    fn pos_x(&self, item: u32) -> String {
        format!(
            "{}",
            coordinate(item, self.size_x, self.padding_x, self.offset_x)
        )
    }

    fn pos_y(&self, item: u32) -> String {
        format!(
            "{}",
            coordinate(item, self.size_y, self.padding_y, self.offset_y)
        )
    }
    fn border(&self) -> String {
        format!("{}", self.border)
    }
}

pub trait Element {
    fn get_color(&self) -> Rgb;
    fn get_border_color(&self) -> Rgb;
}

pub fn tile<D, B, E, W>(
    config: Config,
    data_source: D,
    output: W,
) -> std::result::Result<(), Box<dyn std::error::Error>>
where
    D: Iterator<Item = B>,
    B: Iterator<Item = E>,
    E: Element,
    W: Write,
{
    let mut svg = Writer::new(output);
    svg.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("no"),
    )))?;
    svg.write_event(Event::DocType(BytesText::from_escaped(r#"svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd""#)))?;

    //    let width = 1000;
    //    let height = 1000;
    let ds = Mutex::new(Some(data_source));
    svg.create_element("svg")
        .with_attributes(vec![
            ("version", "1.1"),
            //            ("viewBox", &*format!("0 0 {} {}", width, height)),
            ("xmlns", "http://www.w3.org/2000/svg"),
            ("xmlns:xlink", "http://www.w3.org/1999/xlink"),
        ])
        .write_inner_content(move |svg| {
            let mut y = 0;
            for yval in ds.lock().unwrap().take().unwrap() {
                y += 1;
                let mut x = 0;
                for xval in yval {
                    x += 1;
                    let style = format!(
                        "fill:{};stroke-width:{};stroke:{}",
                        xval.get_color().to_hex_string(),
                        config.border(),
                        xval.get_border_color().to_hex_string()
                    );
                    svg.create_element("rect")
                        .with_attributes(vec![
                            ("x", config.pos_x(x).as_str()),
                            ("y", config.pos_y(y).as_str()),
                            ("width", config.size_x().as_str()),
                            ("height", config.size_y().as_str()),
                            ("style", style.as_str()),
                        ])
                        .write_empty()?;
                }
            }
            Ok(())
        })?;
    Ok(())
}
