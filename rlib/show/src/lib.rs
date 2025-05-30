#![doc = include_str!("../README.md")]

mod impls;
mod traits;

pub use traits::{Show, ShowPretty, ShowSettings, SHOW_SETTINGS};

#[macro_export]
macro_rules! is_show {
    () => {
        std::option_env!("HOUSE").is_some()
    };
}

#[macro_export]
macro_rules! show {
    ($($arg:expr),*) => {
        if is_show!() {
            #[allow(static_mut_refs)]
            let settings = unsafe{ &rlib_show::SHOW_SETTINGS };
            let mut line = format!("[{:>3}] ", line!());
            if settings.colors {
                line = format!("\x1b[34m{}\x1b[0m", line);
            }
            eprint!("{}", line);
            $(
                eprint!(" [{}: {}]", stringify!($arg), $arg.show(settings));
            )*
            eprintln!();
        }
    };
}

#[macro_export]
macro_rules! show_pretty {
    ($arg:expr) => {
        if is_show!() {
            #[allow(static_mut_refs)]
            let settings = unsafe { &rlib_show::SHOW_SETTINGS };
            let mut line = format!("[{:>3}] ", line!());
            let var = stringify!($arg);
            let ident = line.len() + var.len() + 4;
            if settings.colors {
                line = format!("\x1b[34m{}\x1b[0m", line);
            }
            eprint!("{}", line);
            let out = $arg.show_pretty(settings);
            let lines = out.lines().collect::<Vec<_>>();
            for (i, line) in lines.iter().enumerate() {
                if i == 0 {
                    eprint!(" [{}: {}", var, line);
                } else {
                    eprint!("{}{}", " ".repeat(ident), line);
                }
                if i + 1 == lines.len() {
                    eprint!("]");
                }
                eprintln!();
            }
        }
    };
}

#[macro_export]
macro_rules! show_cfg {
    () => {
        if is_show!() {
            unsafe {
                rlib_show::SHOW_SETTINGS = ShowSettings::new();
            }
        }
    };
    ($opt:ident, $value:expr) => {
        if is_show!() {
            let value = $value;
            unsafe {
                rlib_show::SHOW_SETTINGS.$opt = value;
            }
        }
    };
}

#[macro_export]
macro_rules! show_struct {
    ($s:ty, $($field:ident),*) => {
        impl rlib_show::Show for $s {
            fn show(&self, settings: &rlib_show::ShowSettings) -> String {
                let mut fields = Vec::new();
                $(
                    fields.push(format!("{}: {}", stringify!($field), self.$field.show(settings)));
                )*
                format!("{{{}}}", fields.join(", "))
            }
        }
    };
}

#[macro_export]
macro_rules! show_struct_debug {
    ($($s:ty),*) => {
        $(
            impl rlib_show::Show for $s {
                fn show(&self, settings: &rlib_show::ShowSettings) -> String {
                    format!("{:?}", self)
                }
            }
        )*
    };
}
