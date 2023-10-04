use crate::{Config, Element};
use quick_xml::{
    events::{BytesDecl, BytesText, Event},
    Error, Writer,
};
use std::io::Write;

pub(crate) fn base_doc<F, W>(output: W, f: F) -> std::result::Result<(), Box<dyn std::error::Error>>
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

pub(crate) fn write_rect<W: std::io::Write>(
    svg: &mut Writer<W>,
    config: &Config,
    first: u32,
    second: u32,
    ele: &impl Element,
) -> std::result::Result<(), Error> {
    let style = format!(
        "fill:{};stroke-width:{};stroke:{}",
        ele.get_color().to_hex_string(),
        config.border(),
        ele.get_border_color().to_hex_string()
    );
    let (x, y) = config.positions(first, second);

    svg.create_element("rect")
        .with_attributes(vec![
            ("x", x.as_str()),
            ("y", y.as_str()),
            ("rx", config.rounding().as_str()),
            ("ry", config.rounding().as_str()),
            ("width", config.size_x().as_str()),
            ("height", config.size_y().as_str()),
            ("style", style.as_str()),
        ])
        .write_empty()?;
    Ok(())
}

pub(crate) fn write_text<W: std::io::Write>(
    svg: &mut Writer<W>,
    config: &Config,
    x: u32,
    y: u32,
    val: &str,
) -> std::result::Result<(), Error> {
    let (x, y) = config.positions(y, x);
    svg.create_element("text")
        .with_attributes(vec![
            ("x", x.as_str()),
            ("y", y.as_str()),
            ("dominant-baseline", "hanging"),
        ])
        .write_text_content(BytesText::new(val))?;
    Ok(())
}
