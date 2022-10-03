// https://github.com/rust-lang/rust/issues/35853

#[macro_export]
macro_rules! out_impl {
    ($writer:ident, $x:expr) => {
        $writer.write(&$x);
    };
    ($writer:ident, $x:expr, $($xx:tt)*) => {
        $writer.write(&$x);
        $writer.write_char(' ');
        rlib_io::out_impl!($writer, $($xx)*);
    };
}

#[macro_export]
macro_rules! make_output_macro_ {
    ($writer:ident) => {
        make_output_macro_!($writer, $);
    };

    ($writer:ident, $dol:tt) => {
        #[allow(unused_macros)]
        macro_rules! out {
            ($dol($dol x:tt)*) => {
                rlib_io::out_impl!($writer, $dol($dol x)*);
            };
        };
        #[allow(unused_macros)]
        macro_rules! outln {
            ($dol($dol x:tt)*) => {
                rlib_io::out_impl!($writer, $dol($dol x)*);
                $writer.write_char('\n');
            };
        };
    }
}

pub use crate::make_output_macro_ as make_output_macro;
