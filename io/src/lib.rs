pub mod output_macro;
pub mod reader;
pub mod writer;

#[macro_export]
macro_rules! make_io {
    ($reader:ident, $writer:ident) => {
        let mut $reader = rlib_io::reader::Reader::new(&std::io::stdin());
        let mut $writer = rlib_io::writer::Writer::new(&std::io::stdout());

        rlib_io::output_macro::make_output_macro!($writer);
    };
}
