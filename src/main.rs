use std::fs::OpenOptions;
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
        60
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

fn main() {
    let mut val = Vec::new();
    for i in 0..5 {
        let mut column = Vec::new();
        for z in 0..30 {
            column.push(Value::new(i * 10 + z * 10));
        }
        val.push(column.into_iter());
    }
    let config = Config::new().mode(Mode::RowColumn).build();

    let f = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("output.svg")
        .unwrap();
    tile(config.clone(), val.clone().into_iter(), f).unwrap();

    let f = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("output1.svg")
        .unwrap();
    metadata_tile(config, Meta::default(), val.into_iter(), f).unwrap();
}
