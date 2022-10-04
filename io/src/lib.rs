pub mod output_macro;
pub mod reader;
pub mod writer;

pub use output_macro::make_output_macro;
pub use reader::Reader;
pub use writer::Writer;

#[macro_export]
macro_rules! make_io {
    ($reader:ident, $writer:ident) => {
        #[allow(unused_variables)]
        let mut $reader = rlib_io::reader::Reader::new(Box::new(std::io::stdin().lock()));
        #[allow(unused_variables)]
        let mut $writer = rlib_io::writer::Writer::new(Box::new(std::io::stdout().lock()));

        rlib_io::output_macro::make_output_macro!($writer);
    };
}
