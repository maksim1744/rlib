pub struct ShowSettings {
    pub inf_32: u32,
    pub inf_64: u64,
    pub inf_128: u128,
    pub float_precision: usize,
    pub item_width: usize,
    pub colors: bool,
}

impl ShowSettings {
    pub const fn new() -> Self {
        Self {
            inf_32: 10u32.pow(9),
            inf_64: 10u64.pow(18),
            inf_128: 10u128.pow(36),
            float_precision: 9,
            item_width: 0,
            colors: true,
        }
    }
}

impl Default for ShowSettings {
    fn default() -> Self {
        Self::new()
    }
}

pub static mut SHOW_SETTINGS: ShowSettings = ShowSettings::new();

pub trait Show {
    fn show(&self, settings: &ShowSettings) -> String;
}

pub trait ShowPretty: Show {
    fn show_pretty(&self, settings: &ShowSettings) -> String {
        self.show(settings)
    }
}
