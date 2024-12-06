use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use rlib_num_traits::FixedSizeInteger as _;

use crate::traits::{Show, ShowPretty, ShowSettings};

macro_rules! show_int {
    ($tp:ty, $inf_tp:ident, $inf:ident) => {
        impl Show for $tp {
            #[allow(unused_comparisons)]
            fn show(&self, settings: &ShowSettings) -> String {
                if (self.unsigned_abs() as $inf_tp) < settings.$inf {
                    self.to_string()
                } else if *self < 0 {
                    "-inf".into()
                } else {
                    "inf".into()
                }
            }
        }
    };
}

show_int!(i8, u32, inf_32);
show_int!(u8, u32, inf_32);
show_int!(i16, u32, inf_32);
show_int!(u16, u32, inf_32);
show_int!(i32, u32, inf_32);
show_int!(u32, u32, inf_32);
show_int!(i64, u64, inf_64);
show_int!(u64, u64, inf_64);
show_int!(isize, u64, inf_64);
show_int!(usize, u64, inf_64);
show_int!(i128, u128, inf_128);
show_int!(u128, u128, inf_128);

impl Show for f32 {
    fn show(&self, settings: &ShowSettings) -> String {
        format!("{:.precision$}", self, precision = settings.float_precision)
    }
}

impl Show for f64 {
    fn show(&self, settings: &ShowSettings) -> String {
        format!("{:.precision$}", self, precision = settings.float_precision)
    }
}

impl Show for &str {
    fn show(&self, _settings: &ShowSettings) -> String {
        format!("\"{}\"", self)
    }
}

impl Show for str {
    fn show(&self, settings: &ShowSettings) -> String {
        (&self).show(settings)
    }
}

impl Show for String {
    fn show(&self, settings: &ShowSettings) -> String {
        self.as_str().show(settings)
    }
}

impl<T: Show> Show for [T] {
    fn show(&self, settings: &ShowSettings) -> String {
        let mut res = "[".to_string();
        let mut first = true;
        for item in self.iter() {
            if !first {
                res.push_str(", ");
            }
            res.push_str(&format!(
                "{: >width$}",
                item.show(settings),
                width = settings.item_width
            ));
            first = false;
        }
        res.push(']');
        res
    }
}

impl<T: Show, const N: usize> Show for [T; N] {
    fn show(&self, settings: &ShowSettings) -> String {
        self.as_slice().show(settings)
    }
}

impl<T: Show> Show for Vec<T> {
    fn show(&self, settings: &ShowSettings) -> String {
        (self[..]).show(settings)
    }
}

impl<T: Show> ShowPretty for [Vec<T>] {
    fn show_pretty(&self, settings: &ShowSettings) -> String {
        let mut widths: Vec<usize> = Vec::new();
        let mut mat = Vec::new();
        for row in self.iter() {
            mat.push(row.iter().map(|x| x.show(settings)).collect::<Vec<_>>());
            for (i, x) in mat.last().unwrap().iter().enumerate() {
                if i >= widths.len() {
                    widths.push(settings.item_width);
                }
                widths[i] = widths[i].max(x.len());
            }
        }
        let mut lines = Vec::new();
        for (i, row) in mat.iter().enumerate() {
            let pref = if i == 0 { '[' } else { ' ' };
            let suf = if i + 1 == mat.len() { ']' } else { ',' };
            let row = row
                .iter()
                .enumerate()
                .map(|(j, x)| format!("{:>width$}", x, width = widths[j]))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!("{}[{}]{}", pref, row, suf));
        }
        lines.join("\n")
    }
}

macro_rules! show_map {
    ($tp:ty) => {
        impl<K: Show, V: Show> Show for $tp {
            fn show(&self, settings: &ShowSettings) -> String {
                let mut res = "{".to_string();
                let mut first = true;
                for (k, v) in self.iter() {
                    if !first {
                        res.push_str(", ");
                    }
                    res.push_str(&format!(
                        "({: >width$}, {: >width$})",
                        k.show(settings),
                        v.show(settings),
                        width = settings.item_width
                    ));
                    first = false;
                }
                res.push('}');
                res
            }
        }

        impl<K: Show, V: Show> ShowPretty for $tp {
            fn show_pretty(&self, settings: &ShowSettings) -> String {
                let mut widths = [settings.item_width, settings.item_width];
                let mut items = Vec::new();
                for (k, v) in self.iter() {
                    let item = [k.show(settings), v.show(settings)];
                    for i in 0..2 {
                        widths[i] = widths[i].max(item[i].len());
                    }
                    items.push(item);
                }
                let mut lines = Vec::new();
                for (i, row) in items.iter().enumerate() {
                    let pref = if i == 0 { '{' } else { ' ' };
                    let suf = if i + 1 == items.len() { '}' } else { ',' };
                    let row = format!(
                        "{:<k_width$}: {:<v_width$}",
                        row[0],
                        row[1],
                        k_width = widths[0],
                        v_width = widths[1]
                    );
                    lines.push(format!("{}{}{}", pref, row, suf));
                }
                lines.join("\n")
            }
        }
    };
}

show_map!(BTreeMap<K, V>);
show_map!(HashMap<K, V>);

macro_rules! show_set {
    ($tp:ty) => {
        impl<T: Show> Show for $tp {
            fn show(&self, settings: &ShowSettings) -> String {
                let mut res = "{".to_string();
                let mut first = true;
                for item in self.iter() {
                    if !first {
                        res.push_str(", ");
                    }
                    res.push_str(&format!(
                        "{: >width$}",
                        item.show(settings),
                        width = settings.item_width
                    ));
                    first = false;
                }
                res.push('}');
                res
            }
        }
    };
}

show_set!(BTreeSet<T>);
show_set!(HashSet<T>);

macro_rules! show_tuple {
    ($t:ident,) => {};
    ($t1:ident, $($t:ident,)*) => {
        impl<$t1: Show, $($t: Show,)*> Show for ($t1, $($t,)*) {
            fn show(&self, settings: &ShowSettings) -> String {
                let mut res = "(".to_string();
                #[allow(non_snake_case)]
                let ($t1, $($t,)*) = self;
                res.push_str(&$t1.show(settings));
                $(
                    res.push_str(", ");
                    res.push_str(&$t.show(settings));
                )*
                res.push(')');
                res
            }
        }

        show_tuple!($($t,)*);
    }
}

show_tuple!(A, B, C, D, E, F, G, H, I, J, K, L,);
