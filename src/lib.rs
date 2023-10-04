use crate::pieces::{base_doc, write_after_metadata, write_metadata, write_tile};
pub use colorsys::Rgb;
use std::{io::Write, sync::Mutex};

mod config;
pub use config::{Config, Mode};
mod data_traits;
pub use data_traits::{Element, ElementLink, Info, Metadata};
mod pieces;
#[cfg(feature = "year_line")]
mod year;
#[cfg(feature = "year_line")]
pub use chrono::{Datelike, NaiveDate};
#[cfg(feature = "year_line")]
pub use year::{year_line, DateDataSource};

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
        let top_size = metadata.top_size();
        let left_size = metadata.left_size();
        write_metadata(svg, &config, &metadata)?;
        config.set_metadata_first_offset(top_size);
        config.set_metadata_second_offset(left_size);
        let (max_first, max_second) = write_tile(svg, data_source, &config)?;
        write_after_metadata(svg, &config, metadata, max_first, max_second)?;
        Ok(())
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
        write_tile(svg, data_source, &config)?;
        Ok(())
    })?;
    Ok(())
}
