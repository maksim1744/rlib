#![doc = include_str!("../README.md")]

#[macro_export]
macro_rules! _rec_lambda_2_ {
    ($name:ident, $ret:ty, [$($const_var_name:ident: $const_var_type:ty,)*], [$($mut_var_name:ident: $mut_var_type:ty,)*], [$($arg:ident: $arg_type:ty,)*], {$($body:tt)*}, $dol:tt) => {
        {
            fn _lambda_name_($($arg:$arg_type,)* $($const_var_name:&$const_var_type,)* $($mut_var_name:&mut $mut_var_type,)*) -> $ret {
                macro_rules! $name {
                    ($dol xf:expr $dol(,$dol x:expr)*) => {
                        $name!($dol xf, $dol($dol x,)*)
                    };
                    ($dol($dol x:expr,)*) => {
                        _lambda_name_($dol ($dol x,)* $($const_var_name,)* $($mut_var_name,)*)
                    }
                }
                $($body)*
            }
            |$($arg: $arg_type,)*| {
                _lambda_name_($($arg,)* $(&$const_var_name,)* $(&mut $mut_var_name,)*)
            }
        }
    };
}

#[macro_export]
macro_rules! _rec_lambda_1_ {
    ($name:ident, [$($const_var_name:ident: $const_var_type:ty,)*], [$($mut_var_name:ident: $mut_var_type:ty,)*], [$($arg:ident: $arg_type:ty,)*], $var:ident:$var_type:ty, $($rem:tt)*) => {
        $crate::_rec_lambda_1_!($name, [$($const_var_name:$const_var_type,)*], [$($mut_var_name:$mut_var_type,)*], [$($arg:$arg_type,)* $var:$var_type,], $($rem)*)
    };
    ($name:ident, [$($const_var_name:ident: $const_var_type:ty,)*], [$($mut_var_name:ident: $mut_var_type:ty,)*], [$($arg:ident: $arg_type:ty,)*], $var:ident:$var_type:ty| -> $ret:ty {$($rem:tt)*}) => {
        $crate::_rec_lambda_2_!($name, $ret, [$($const_var_name:$const_var_type,)*], [$($mut_var_name:$mut_var_type,)*], [$($arg:$arg_type,)* $var:$var_type,], {$($rem)*}, $)
    };
    ($name:ident, [$($const_var_name:ident: $const_var_type:ty,)*], [$($mut_var_name:ident: $mut_var_type:ty,)*], [$($arg:ident: $arg_type:ty,)*], $var:ident:$var_type:ty| {$($rem:tt)*}) => {
        $crate::_rec_lambda_2_!($name, (), [$($const_var_name:$const_var_type,)*], [$($mut_var_name:$mut_var_type,)*], [$($arg:$arg_type,)* $var:$var_type,], {$($rem)*}, $)
    };
}

#[macro_export]
macro_rules! _rec_lambda_0_ {
    ($name:ident, [$($const_var_name:ident: $const_var_type:ty,)*], [$($mut_var_name:ident: $mut_var_type:ty,)*], [$($arg:ident: $arg_type:ty,)*], $var:ident:&mut $var_type:ty, $($rem:tt)*) => {
        $crate::_rec_lambda_0_!($name, [$($const_var_name:$const_var_type,)*], [$var:$var_type, $($mut_var_name:$mut_var_type,)*], [$($arg:$arg_type,)*], $($rem)*)
    };
    ($name:ident, [$($const_var_name:ident: $const_var_type:ty,)*], [$($mut_var_name:ident: $mut_var_type:ty,)*], [$($arg:ident: $arg_type:ty,)*], $var:ident:&$var_type:ty, $($rem:tt)*) => {
        $crate::_rec_lambda_0_!($name, [$var:$var_type, $($const_var_name:$const_var_type,)*], [$($mut_var_name:$mut_var_type,)*], [$($arg:$arg_type,)*], $($rem)*)
    };
    ($name:ident, [$($const_var_name:ident: $const_var_type:ty,)*], [$($mut_var_name:ident: $mut_var_type:ty,)*], [$($arg:ident: $arg_type:ty,)*], $var:ident:&mut $var_type:ty| {|$($rem:tt)*}) => {
        $crate::_rec_lambda_1_!($name, [$($const_var_name:$const_var_type,)*], [$var:$var_type, $($mut_var_name:$mut_var_type,)*], [$($arg:$arg_type,)*], $($rem)*)
    };
    ($name:ident, [$($const_var_name:ident: $const_var_type:ty,)*], [$($mut_var_name:ident: $mut_var_type:ty,)*], [$($arg:ident: $arg_type:ty,)*], $var:ident:&$var_type:ty| {|$($rem:tt)*}) => {
        $crate::_rec_lambda_1_!($name, [$var:$var_type, $($const_var_name:$const_var_type,)*], [$($mut_var_name:$mut_var_type,)*], [$($arg:$arg_type,)*], $($rem)*)
    };
}

#[macro_export]
macro_rules! rec_lambda {
    ($name:ident, || {|$($rem:tt)*}) => {
        $crate::_rec_lambda_1_!($name, [], [], [], $($rem)*)
    };
    ($name:ident, |$($rem:tt)*) => {
        $crate::_rec_lambda_0_!($name, [], [], [], $($rem)*)
    };
}
