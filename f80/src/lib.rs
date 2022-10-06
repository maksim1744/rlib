// Mostly was made by Hegdahl (https://github.com/Hegdahl)

// Make sure to call f80_init() in the beginning of fn main(),
// it enables f80 on windows (for example, on codeforces)

// A lot of https://doc.rust-lang.org/std/primitive.f64.html is yet to be implemented

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(align(16))]
#[allow(non_camel_case_types)]
pub struct f80([u8; 10]);

pub fn f80_init() {
    #[cfg(target_family = "windows")]
    unsafe {
        core::arch::asm! {
            "finit"
        }
    }
}

macro_rules! define_f80_binary_op {
    ($trait:ident, $fun:ident, $asm:literal) => {
        impl $trait for f80 {
            type Output = f80;
            fn $fun(self, rhs: Self) -> Self::Output {
                let mut res = core::mem::MaybeUninit::<f80>::uninit();
                unsafe {
                    core::arch::asm! {
                        "fld     TBYTE PTR [{0}]",
                        "fld     TBYTE PTR [{1}]",
                        $asm,
                        "fstp    TBYTE PTR [{2}]",
                        in(reg) self.0.as_ptr(),
                        in(reg) rhs.0.as_ptr(),
                        in(reg) res.as_mut_ptr(),
                        options(nostack)
                    }
                    res.assume_init()
                }
            }
        }
    };
}

macro_rules! define_f80_unary_op {
    ($trait:ident, $fun:ident, $asm:literal) => {
        impl $trait for f80 {
            type Output = f80;
            fn $fun(self) -> Self::Output {
                let mut res = core::mem::MaybeUninit::<f80>::uninit();
                unsafe {
                    core::arch::asm! {
                        "fld     TBYTE PTR [{0}]",
                        $asm,
                        "fstp    TBYTE PTR [{1}]",
                        in(reg) self.0.as_ptr(),
                        in(reg) res.as_mut_ptr(),
                        options(nostack)
                    }
                    res.assume_init()
                }
            }
        }
    };
}

macro_rules! define_f80_assign_op {
    ($trait:ident, $fun:ident, $no_assign_fun:ident) => {
        impl $trait for f80 {
            fn $fun(&mut self, rhs: f80) {
                *self = self.$no_assign_fun(rhs);
            }
        }
    };
}

define_f80_binary_op!(Add, add, "faddp   st(1), st");
define_f80_binary_op!(Sub, sub, "fsubp   st(1), st");
define_f80_binary_op!(Mul, mul, "fmulp   st(1), st");
define_f80_binary_op!(Div, div, "fdivp   st(1), st");

define_f80_assign_op!(AddAssign, add_assign, add);
define_f80_assign_op!(SubAssign, sub_assign, sub);
define_f80_assign_op!(MulAssign, mul_assign, mul);
define_f80_assign_op!(DivAssign, div_assign, div);

define_f80_unary_op!(Neg, neg, "fchs");

impl PartialOrd<f80> for f80 {
    fn lt(&self, rhs: &f80) -> bool {
        let mut res = std::mem::MaybeUninit::<u32>::uninit();
        unsafe {
            let e: u32;
            core::arch::asm! {
                "fld     TBYTE PTR [{0}]",
                "fld     TBYTE PTR [{1}]",
                "fcomip  st, st(1)",
                "fstp    st(0)",
                "seta    al",
                in(reg) self.0.as_ptr(),
                in(reg) rhs.0.as_ptr(),
                out("eax") e,
                options(nostack)
            }
            *res.as_mut_ptr() = e;
            (res.assume_init() & 1) > 0
        }
    }

    fn gt(&self, rhs: &f80) -> bool {
        rhs.lt(self)
    }

    fn le(&self, rhs: &f80) -> bool {
        !self.gt(rhs)
    }

    fn ge(&self, rhs: &f80) -> bool {
        !self.lt(rhs)
    }

    fn partial_cmp(&self, rhs: &f80) -> Option<Ordering> {
        // same as f64
        match (*self <= *rhs, *self >= *rhs) {
            (false, false) => None,
            (false, true) => Some(Ordering::Greater),
            (true, false) => Some(Ordering::Less),
            (true, true) => Some(Ordering::Equal),
        }
    }
}

impl f80 {
    pub const ZERO: f80 = f80([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    pub const ONE: f80 = f80([0, 0, 0, 0, 0, 0, 0, 128, 255, 63]);

    pub fn abs(self) -> f80 {
        if self < f80::from(0.) {
            -self
        } else {
            self
        }
    }

    pub fn min(self, rhs: f80) -> f80 {
        let mut res = core::mem::MaybeUninit::<f80>::uninit();
        unsafe {
            core::arch::asm! {
                "fld     TBYTE PTR [{0}]",
                "fld     TBYTE PTR [{1}]",
                "fucomi  st, st(1)",
                "fcmovnbe st, st(1)",
                "fstp    st(1)",
                "fstp    TBYTE PTR [{2}]",
                in(reg) self.0.as_ptr(),
                in(reg) rhs.0.as_ptr(),
                in(reg) res.as_mut_ptr(),
                options(nostack)
            }
            res.assume_init()
        }
    }

    pub fn max(self, rhs: f80) -> f80 {
        let mut res = core::mem::MaybeUninit::<f80>::uninit();
        unsafe {
            core::arch::asm! {
                "fld     TBYTE PTR [{0}]",
                "fld     TBYTE PTR [{1}]",
                "fucomi  st, st(1)",
                "fcmovbe st, st(1)",
                "fstp    st(1)",
                "fstp    TBYTE PTR [{2}]",
                in(reg) self.0.as_ptr(),
                in(reg) rhs.0.as_ptr(),
                in(reg) res.as_mut_ptr(),
                options(nostack)
            }
            res.assume_init()
        }
    }
}

impl From<f80> for f64 {
    fn from(f: f80) -> Self {
        let mut res = core::mem::MaybeUninit::<f64>::uninit();
        unsafe {
            core::arch::asm! {
                "fld     TBYTE PTR [{0}]",
                "fstp    QWORD PTR [{1}]",
                in(reg) f.0.as_ptr(),
                in(reg) res.as_mut_ptr(),
                options(nostack)
            }
            res.assume_init()
        }
    }
}

impl From<f64> for f80 {
    fn from(f: f64) -> Self {
        let mut res = core::mem::MaybeUninit::<f80>::uninit();
        unsafe {
            core::arch::asm! {
                "fld     QWORD PTR [{0}]",
                "fstp    TBYTE PTR [{1}]",
                in(reg) &f as *const _,
                in(reg) res.as_mut_ptr(),
                options(nostack)
            }
            res.assume_init()
        }
    }
}

impl Default for f80 {
    fn default() -> Self {
        f80::from(0.)
    }
}

impl std::fmt::Display for f80 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f64::from(*self).fmt(f)
    }
}

impl std::fmt::Debug for f80 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f64::from(*self).fmt(f)
    }
}
