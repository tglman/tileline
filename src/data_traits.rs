pub use colorsys::Rgb;

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
    //    fn right_size(&self) -> u32;
    //    fn bottom_size(&self) -> u32;
    fn left(&self) -> Option<IT>;
    fn top(&self) -> Option<IT>;
    fn right(&self) -> Option<IT>;
    fn bottom(&self) -> Option<IT>;
}

impl Element for (Rgb, Rgb, Option<(String, String)>) {
    fn get_color(&self) -> Rgb {
        self.0.clone()
    }

    fn get_border_color(&self) -> Rgb {
        self.1.clone()
    }

    fn get_link(&self) -> Option<Box<dyn ElementLink>> {
        if let Some(s) = &self.2 {
            Some(Box::new(s.clone()))
        } else {
            None
        }
    }
}

impl ElementLink for (String, String) {
    fn link(&self) -> String {
        self.1.clone()
    }

    fn title(&self) -> String {
        self.0.clone()
    }
}

impl Info for (u32, String) {
    fn block_count(&self) -> u32 {
        self.0
    }

    fn label(&self) -> &str {
        &self.1
    }
}
