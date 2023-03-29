use builder_pattern::Builder;
pub use colorsys::Rgb;
use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::{Error, Writer};
use std::{io::Write, sync::Mutex};

#[derive(Builder, Clone)]
pub struct Config {
    #[default(0)]
    offset_x: u32,
    #[default(0)]
    offset_y: u32,
    #[default(15)]
    size_x: u32,
    #[default(15)]
    size_y: u32,
    #[default(5)]
    padding_x: u32,
    #[default(5)]
    padding_y: u32,
    #[default(1)]
    border: u32,
    #[default(3)]
    rounding: u32,
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
    fn rounding(&self) -> String {
        format!("{}", self.rounding)
    }
}

pub trait Element {
    fn get_color(&self) -> Rgb;
    fn get_border_color(&self) -> Rgb;
    fn get_link(&self) -> Option<Box<dyn ElementLink>>;
}
pub trait ElementLink {
    fn link(&self) -> String;
    fn title(&self) -> String;
}

pub trait Info {
    fn block_count(&self) -> u32;
    fn label(&self) -> &str;
}

pub trait Metadata<IT, IF>
where
    IT: Iterator<Item = IF>,
    IF: Info,
{
    fn left_size(&self) -> u32;
    fn top_size(&self) -> u32;
    fn right_size(&self) -> u32;
    fn bottom_size(&self) -> u32;
    fn left(&self) -> Option<IT>;
    fn top(&self) -> Option<IT>;
    fn right(&self) -> Option<IT>;
    fn bottom(&self) -> Option<IT>;
}

pub fn metadata_tile<D, B, E, W, M, MIT, MIN>(
    config: Config,
    metadata: M,
    data_source: D,
    output: W,
) -> std::result::Result<(), Box<dyn std::error::Error>>
where
    D: Iterator<Item = B>,
    B: Iterator<Item = E>,
    E: Element,
    W: Write,
    M: Metadata<MIT, MIN>,
    MIT: Iterator<Item = MIN>,
    MIN: Info,
{
    let ds = Mutex::new(Some((config, data_source, metadata)));
    base_doc(output, move |svg| {
        let (mut config, data_source, metadata) = ds.lock().unwrap().take().unwrap();
        let left_size = metadata.left_size();
        let top_size = metadata.top_size();
        write_metadata(svg, &config, metadata)?;
        config.offset_x += left_size;
        config.offset_y += top_size;
        write_tile(svg, data_source, &config)
    })?;
    Ok(())
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
    let ds = Mutex::new(Some(data_source));
    base_doc(output, move |svg| {
        let data_source = ds.lock().unwrap().take().unwrap();
        write_tile(svg, data_source, &config)
    })?;
    Ok(())
}

fn base_doc<F, W>(output: W, f: F) -> std::result::Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&mut Writer<W>) -> std::result::Result<(), Error>,
    W: Write,
{
    let mut svg = Writer::new(output);
    svg.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("no"),
    )))?;
    svg.write_event(Event::DocType(BytesText::from_escaped(r#"svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd""#)))?;

    svg.create_element("svg")
        .with_attributes(vec![
            ("version", "1.1"),
            //            ("viewBox", &*format!("0 0 {} {}", width, height)),
            ("xmlns", "http://www.w3.org/2000/svg"),
            ("xmlns:xlink", "http://www.w3.org/1999/xlink"),
        ])
        .write_inner_content(f)?;
    Ok(())
}

fn write_metadata<W, M, MIT, MIN>(
    svg: &mut Writer<W>,
    config: &Config,
    metadata: M,
) -> std::result::Result<(), Error>
where
    M: Metadata<MIT, MIN>,
    MIT: Iterator<Item = MIN>,
    MIN: Info,
    W: Write,
{
    let top_size = metadata.top_size();
    let left_size = metadata.left_size();
    if let Some(iter) = metadata.left() {
        let mut y = 0;
        let mut c = config.clone();
        c.offset_y += top_size;
        for info in iter {
            let block = info.block_count() / 2;
            y += block;
            write_text(svg, &c, 1, y, info.label())?;
            y += block;
        }
    }

    if let Some(iter) = metadata.top() {
        let mut x = 0;
        let mut c = config.clone();
        c.offset_x += left_size;
        for info in iter {
            let block = info.block_count();
            write_text(svg, &c, x, 1, info.label())?;
            x += block;
        }
    }

    Ok(())
}

fn write_tile<W, D, B, E>(
    svg: &mut Writer<W>,
    data_source: D,
    config: &Config,
) -> std::result::Result<(), Error>
where
    D: Iterator<Item = B>,
    B: Iterator<Item = E>,
    E: Element,
    W: Write,
{
    let mut y = 0;
    for yval in data_source {
        let mut x = 0;
        for xval in yval {
            if let Some(l) = xval.get_link() {
                svg.create_element("a")
                    .with_attributes(vec![
                        ("xlink:href", l.link().as_str()),
                        ("xlink:title", l.title().as_str()),
                    ])
                    .write_inner_content(|svg| {
                        write_rect(svg, &config, x, y, &xval)?;
                        Ok(())
                    })?;
            } else {
                write_rect(svg, &config, x, y, &xval)?;
            }
            x += 1;
        }
        y += 1;
    }
    Ok(())
}

fn write_rect<W: std::io::Write>(
    svg: &mut Writer<W>,
    config: &Config,
    x: u32,
    y: u32,
    ele: &impl Element,
) -> std::result::Result<(), Error> {
    let style = format!(
        "fill:{};stroke-width:{};stroke:{}",
        ele.get_color().to_hex_string(),
        config.border(),
        ele.get_border_color().to_hex_string()
    );

    svg.create_element("rect")
        .with_attributes(vec![
            ("x", config.pos_x(x).as_str()),
            ("y", config.pos_y(y).as_str()),
            ("rx", config.rounding().as_str()),
            ("ry", config.rounding().as_str()),
            ("width", config.size_x().as_str()),
            ("height", config.size_y().as_str()),
            ("style", style.as_str()),
        ])
        .write_empty()?;
    Ok(())
}

fn write_text<W: std::io::Write>(
    svg: &mut Writer<W>,
    config: &Config,
    x: u32,
    y: u32,
    val: &str,
) -> std::result::Result<(), Error> {
    svg.create_element("text")
        .with_attributes(vec![
            ("x", config.pos_x(x).as_str()),
            ("y", config.pos_y(y).as_str()),
        ])
        .write_text_content(BytesText::new(val))?;
    Ok(())
}
