use builder_pattern::Builder;

#[derive(Default, Clone)]
pub enum Mode {
    #[default]
    RowColumn,
    ColumnRow,
}

#[derive(Builder, Clone)]
pub struct Config {
    #[default(0)]
    #[public]
    offset_x: u32,
    #[default(0)]
    #[public]
    offset_y: u32,
    #[default(15)]
    #[public]
    size_x: u32,
    #[default(15)]
    #[public]
    size_y: u32,
    #[default(5)]
    padding_x: u32,
    #[default(5)]
    #[public]
    padding_y: u32,
    #[default(1)]
    #[public]
    border: u32,
    #[default(3)]
    #[public]
    rounding: u32,
    #[default(Mode::RowColumn)]
    #[public]
    mode: Mode,
}

fn coordinate(base: u32, size: u32, padding: u32, offset: u32) -> u32 {
    offset + (base * (size + padding))
}

impl Config {
    pub(crate) fn size_x(&self) -> String {
        format!("{}", self.size_x)
    }
    pub(crate) fn size_y(&self) -> String {
        format!("{}", self.size_x)
    }

    fn coordinate_x(&self, value: u32) -> u32 {
        coordinate(value, self.size_x, self.padding_x, self.offset_x)
    }

    fn coordinate_y(&self, value: u32) -> u32 {
        coordinate(value, self.size_y, self.padding_y, self.offset_y)
    }
    pub(crate) fn positions(&self, first: u32, second: u32) -> (String, String) {
        let (x, y) = match self.mode {
            Mode::RowColumn => (second, first),
            Mode::ColumnRow => (first, second),
        };
        let xc = self.coordinate_x(x);
        let yc = self.coordinate_y(y);
        let xs = format!("{}", xc);
        let ys = format!("{}", yc);
        (xs, ys)
    }

    pub(crate) fn set_metadata_first_offset(&mut self, item: u32) {
        match self.mode {
            Mode::RowColumn => self.offset_y += item,
            Mode::ColumnRow => self.offset_x += item,
        };
    }

    pub(crate) fn set_metadata_after_first_offset(&mut self, max_first: u32) {
        match self.mode {
            Mode::RowColumn => self.offset_y = self.coordinate_y(max_first),
            Mode::ColumnRow => self.offset_x = self.coordinate_x(max_first),
        };
    }
    pub(crate) fn set_metadata_second_offset(&mut self, item: u32) {
        match self.mode {
            Mode::RowColumn => self.offset_x += item,
            Mode::ColumnRow => self.offset_y += item,
        };
    }
    pub(crate) fn set_metadata_after_second_offset(&mut self, max_second: u32) {
        match self.mode {
            Mode::RowColumn => self.offset_x = self.coordinate_x(max_second),
            Mode::ColumnRow => self.offset_y = self.coordinate_y(max_second),
        };
    }
    pub(crate) fn border(&self) -> String {
        format!("{}", self.border)
    }
    pub(crate) fn rounding(&self) -> String {
        format!("{}", self.rounding)
    }
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}
