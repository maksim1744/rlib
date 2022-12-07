pub mod output_macro;
pub mod reader;
pub mod writer;

pub use output_macro::make_output_macro;
pub use reader::{Readable, Reader};
pub use writer::{Writable, Writer};

#[macro_export]
macro_rules! make_io {
    ($reader:ident, $writer:ident) => {
        let _stdin_ = std::io::stdin();
        #[allow(unused_variables)]
        let mut $reader = rlib_io::reader::Reader::new(Box::new(_stdin_.lock()));
        let _stdout_ = std::io::stdout();
        #[allow(unused_variables)]
        let mut $writer = rlib_io::writer::Writer::new(Box::new(_stdout_.lock()));

        rlib_io::output_macro::make_output_macro!($reader, $writer);
    };
}
