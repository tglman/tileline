use crate::{Config, Element, Info, Metadata};
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

pub(crate) fn write_metadata<W, M, MIT, MIN>(
    svg: &mut Writer<W>,
    config: &Config,
    metadata: &M,
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
        c.set_metadata_first_offset(top_size);
        for info in iter {
            write_text(svg, &c, 0, y, info.label())?;
            y += info.block_count();
        }
    }

    if let Some(iter) = metadata.top() {
        let mut x = 0;
        let mut c = config.clone();
        c.set_metadata_second_offset(left_size);
        for info in iter {
            write_text(svg, &c, x, 0, info.label())?;
            x += info.block_count();
        }
    }

    Ok(())
}

pub(crate) fn write_tile<W, D, B, E>(
    svg: &mut Writer<W>,
    data_source: D,
    config: &Config,
) -> std::result::Result<(u32, u32), Error>
where
    D: Iterator<Item = B>,
    B: Iterator<Item = E>,
    E: Element,
    W: Write,
{
    let mut max_second = 0;
    let mut first = 0;
    for yval in data_source {
        let mut second = 0;
        for xval in yval {
            if let Some(l) = xval.get_link() {
                svg.create_element("a")
                    .with_attributes(vec![
                        ("xlink:href", l.link().as_str()),
                        ("xlink:title", l.title().as_str()),
                    ])
                    .write_inner_content(|svg| {
                        write_rect(svg, &config, first, second, &xval)?;
                        Ok(())
                    })?;
            } else {
                write_rect(svg, &config, first, second, &xval)?;
            }
            second += 1;
            if second > max_second {
                max_second = second;
            }
        }
        first += 1;
    }
    Ok((first, max_second))
}

pub(crate) fn write_after_metadata<W, M, MIT, MIN>(
    svg: &mut Writer<W>,
    config: &Config,
    metadata: M,
    max_first: u32,
    max_second: u32,
) -> std::result::Result<(), Error>
where
    M: Metadata<MIT, MIN>,
    MIT: Iterator<Item = MIN>,
    MIN: Info,
    W: Write,
{
    if let Some(iter) = metadata.right() {
        let mut y = 0;
        let mut c = config.clone();
        c.set_metadata_after_second_offset(max_second);
        for info in iter {
            let block = info.block_count();
            write_text(svg, &c, 0, y, info.label())?;
            y += block;
        }
    }

    if let Some(iter) = metadata.bottom() {
        let mut x = 0;
        let mut c = config.clone();
        c.set_metadata_after_first_offset(max_first);
        for info in iter {
            let block = info.block_count();
            write_text(svg, &c, x, 0, info.label())?;
            x += block;
        }
    }

    Ok(())
}
