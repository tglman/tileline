use tileline::{metadata_tile, tile, Config, Element, ElementLink, Info, Metadata, Mode, Rgb};

#[derive(Clone)]
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
        Some(Box::new(ElementLinkImpl(
            "https://tglman.com".to_owned(),
            "Tglman.com".to_owned(),
        )))
    }
}
struct ElementLinkImpl(String, String);
impl ElementLink for ElementLinkImpl {
    fn link(&self) -> String {
        self.0.clone()
    }
    fn title(&self) -> String {
        self.1.clone()
    }
}

#[derive(Default)]
struct Meta {}

impl Metadata<std::vec::IntoIter<MetaInfo>, MetaInfo> for Meta {
    fn left_size(&self) -> u32 {
        30
    }

    fn top_size(&self) -> u32 {
        20
    }

    fn left(&self) -> Option<std::vec::IntoIter<MetaInfo>> {
        Some(
            (0..2)
                .into_iter()
                .map(|_| MetaInfo::default())
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }

    fn top(&self) -> Option<std::vec::IntoIter<MetaInfo>> {
        Some(
            (0..15)
                .into_iter()
                .map(|_| MetaInfo::default())
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }

    fn right(&self) -> Option<std::vec::IntoIter<MetaInfo>> {
        Some(
            (0..2)
                .into_iter()
                .map(|_| MetaInfo::default())
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }

    fn bottom(&self) -> Option<std::vec::IntoIter<MetaInfo>> {
        Some(
            (0..15)
                .into_iter()
                .map(|_| MetaInfo::default())
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }
}

#[derive(Default)]
struct MetaInfo {}
impl Info for MetaInfo {
    fn block_count(&self) -> u32 {
        2
    }

    fn label(&self) -> &str {
        "label"
    }
}

#[test]
fn test_simple() {
    let mut val = Vec::new();
    for i in 0..5 {
        let mut column = Vec::new();
        for z in 0..30 {
            column.push(Value::new(i * 10 + z * 10));
        }
        val.push(column.into_iter());
    }

    let config = Config::new().build();
    let mut out = Vec::new();
    tile(config.clone(), val.clone().into_iter(), &mut out).unwrap();
    assert_eq!(out, std::fs::read("./fixtures/simple.svg").unwrap().to_vec());
}

#[test]
fn test_simple_column_row() {
    let mut val = Vec::new();
    for i in 0..5 {
        let mut column = Vec::new();
        for z in 0..30 {
            column.push(Value::new(i * 10 + z * 10));
        }
        val.push(column.into_iter());
    }

    let config = Config::new().mode(Mode::ColumnRow).build();
    let mut out = Vec::new();
    tile(config.clone(), val.clone().into_iter(), &mut out).unwrap();
    assert_eq!(out, std::fs::read("./fixtures/simple_column_row.svg").unwrap().to_vec());
}

#[test]
fn test_metadata() {
    let mut val = Vec::new();
    for i in 0..5 {
        let mut column = Vec::new();
        for z in 0..30 {
            column.push(Value::new(i * 10 + z * 10));
        }
        val.push(column.into_iter());
    }

    let config = Config::new().build();
    let mut out = Vec::new();
    metadata_tile(config, Meta::default(), val.into_iter(), &mut out).unwrap();
    assert_eq!(out, std::fs::read("./fixtures/simple_metadata.svg").unwrap().to_vec());
}

#[test]
#[cfg(feature = "year_line")]
fn test_year_line() {
    use chrono::{Datelike, NaiveDate};
    struct YearDatasource {}
    struct DateElement {
        color: Rgb,
        date: NaiveDate,
    }
    impl Element for DateElement {
        fn get_color(&self) -> Rgb {
            self.color.clone()
        }

        fn get_border_color(&self) -> Rgb {
            self.color.clone()
        }

        fn get_link(&self) -> Option<Box<dyn ElementLink>> {
            Some(Box::new(ElementLinkImpl(format!("{}", self.date), "bbb".to_owned())))
        }
    }
    impl tileline::DateDataSource<DateElement> for YearDatasource {
        fn get_element(&self, data: chrono::NaiveDate) -> DateElement {
            if data.day() % 2 == 0 {
                DateElement {
                    color: Rgb::new(0.0, 0.0, 100.0, None),
                    date: data.clone(),
                }
            } else {
                DateElement {
                    color: Rgb::new(0.0, 100.0, 0.0, None),
                    date: data.clone(),
                }
            }
        }
    }

    let config = Config::new().build();
    let mut out = Vec::new();
    tileline::year_line(2023, YearDatasource {}, &mut out, config).unwrap();
    assert_eq!(out, std::fs::read("./fixtures/year_line.svg").unwrap().to_vec());
}
