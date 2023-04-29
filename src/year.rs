use chrono::{Datelike, NaiveDate};
use std::rc::Rc;
use colorsys::Rgb;
use crate::{Element, Mode, ElementLink, Info, Metadata, Config, metadata_tile};
use std::io::Write;

pub trait DateDataSource<E: Element> {
    fn get_element(&self, data: NaiveDate) -> E;
}

struct Year<E: Element> {
    date: Box<dyn Iterator<Item = NaiveDate>>,
    data_source: Rc<dyn DateDataSource<E>>,
    year: i32,
}
impl<E: Element> Year<E> {
    fn new(year: i32, data_source: Rc<dyn DateDataSource<E>>) -> Self {
        Self {
            date: Box::new(
                NaiveDate::from_ymd_opt(year, 1, 1)
                    .unwrap()
                    .iter_weeks()
                    .take_while(move |x| x.year() == year),
            ),
            data_source,
            year,
        }
    }
}

impl<E: Element> Iterator for Year<E> {
    type Item = Week<E>;
    fn next(&mut self) -> Option<Self::Item> {
        let ds = self.data_source.clone();
        self.date.next().map(|w| Week::new(w, self.year, ds))
    }
}

struct Week<E: Element> {
    date: Box<dyn Iterator<Item = NaiveDate>>,
    data_source: Rc<dyn DateDataSource<E>>,
    empty: u32,
}
impl<E: Element> Week<E> {
    fn new(date: NaiveDate, year: i32, data_source: Rc<dyn DateDataSource<E>>) -> Self {
        let weekday = if date.iso_week().year() != year {
            let rem = 7 - date.weekday().number_from_sunday();
            date.checked_add_days(chrono::Days::new(rem as u64))
                .unwrap()
        } else {
            date.checked_add_days(chrono::Days::new(7)).unwrap()
        };
        let empty = if date.iso_week().year() != year {
            date.weekday().number_from_sunday()
        } else {
            0
        };
        Self {
            date: Box::new(date.iter_days().take_while(move |d| d != &weekday)),
            empty,
            data_source,
        }
    }
}

impl<E: Element> Iterator for Week<E> {
    type Item = WrapperElement<E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.empty > 0 {
            self.empty -= 1;
            Some(WrapperElement::None)
        } else {
            let ds = self.data_source.clone();
            self.date
                .next()
                .map(|d| WrapperElement::Real(ds.get_element(d)))
        }
    }
}

enum WrapperElement<E> {
    Real(E),
    None,
}
impl<E: Element> Element for WrapperElement<E> {
    fn get_color(&self) -> Rgb {
        match self {
            Self::Real(e) => e.get_color(),
            Self::None => Rgb::new(255.0, 255.0, 255.0, Some(255.0)),
        }
    }

    fn get_border_color(&self) -> Rgb {
        match self {
            Self::Real(e) => e.get_border_color(),
            Self::None => Rgb::new(255.0, 255.0, 255.0, Some(255.0)),
        }
    }

    fn get_link(&self) -> Option<Box<dyn ElementLink>> {
        match self {
            Self::Real(e) => e.get_link(),
            Self::None => None,
        }
    }
}

struct YearMetadata {}
struct YearInfo {
    block_count: u32,
    label: String,
}
impl YearInfo {
    fn day(day: &str) -> Self {
        Self {
            block_count: 2,
            label: day.to_owned(),
        }
    }

    fn month(day: &str) -> Self {
        Self {
            block_count: 4,
            label: day.to_owned(),
        }
    }
}

impl Info for YearInfo {
    fn block_count(&self) -> u32 {
        self.block_count
    }

    fn label(&self) -> &str {
        &self.label
    }
}

impl Metadata<std::vec::IntoIter<YearInfo>, YearInfo> for YearMetadata {
    fn left_size(&self) -> u32 {
        40
    }

    fn top_size(&self) -> u32 {
        40
    }

    fn left(&self) -> Option<std::vec::IntoIter<YearInfo>> {
        Some(
            vec![
                YearInfo::month("Jan"),
                YearInfo::month("Feb"),
                YearInfo::month("Mar"),
                YearInfo::month("Apr"),
                YearInfo::month("May"),
                YearInfo::month("Jun"),
                YearInfo::month("Jul"),
                YearInfo::month("Aug"),
                YearInfo::month("Sep"),
                YearInfo::month("Oct"),
                YearInfo::month("Nov"),
                YearInfo::month("Dec"),
            ]
            .into_iter(),
        )
    }

    fn top(&self) -> Option<std::vec::IntoIter<YearInfo>> {
        Some(vec![YearInfo::day("S"), YearInfo::day("T"), YearInfo::day("S")].into_iter())
    }

    fn right(&self) -> Option<std::vec::IntoIter<YearInfo>> {
        None
    }

    fn bottom(&self) -> Option<std::vec::IntoIter<YearInfo>> {
        None
    }
}

pub fn year_line<W, D, E>(
    year: i32,
    data_source: D,
    output: W,
    mut config: Config,
) -> std::result::Result<(), Box<dyn std::error::Error>>
where
    D: DateDataSource<E> + 'static,
    E: Element,
    W: Write,
{
    config.mode = Mode::ColumnRow;
    let metadata = YearMetadata {};
    metadata_tile(
        config,
        metadata,
        Year::new(year, Rc::new(data_source)),
        output,
    )?;
    Ok(())
}
