pub(crate) mod resize;
pub(crate) mod trunc;

pub(crate) use crate::doc::*;

macro_rules! decimal_type_doc {
    ($bits: literal $($sign: ident)?) => {
        concat!(
            "Strictly exact precision fixed-size", $(concat!(" ", doc::text_sign!($sign)), )? " decimal number ",
            "with ", $bits, "-bit integer for decimal digits."
        )
    };
}

pub(crate) use decimal_type_doc;

macro_rules! type_str {
    ($bits: literal $($sign: ident)?) => {
        concat!($(stringify!($sign),)? "D", $bits)
    };
}

pub(crate) use type_str;

macro_rules! link_type_str {
    ($bits: literal $($sign: ident)?) => {
        concat!("[`", doc::type_str!($bits $($sign)?), "`](crate::", doc::type_str!($bits $($sign)?), ")")
    };
}

pub(crate) use link_type_str;

macro_rules! m {
    ($bits: literal $($sign: ident)?) => {
        concat!($(doc::small_sign!($sign),)? "dec", $bits, "!")
    };
}

pub(crate) use m;

macro_rules! decimal_operation_panics {
    ($op: literal) => {
        concat!(
            "### debug mode\n\n",
            "This method will panic if ",
            $op,
            " performs with some [Exceptional condition](crate#signaling-flags-and-trap-enablers) and corresponding [Signals] in the [Context]
            is trapped by trap-enabler.
            \n\n",
            "### release mode\n\n",
            "In release mode panic will not occur and result will be [`NaN`](crate#nan).\n\n"
        )
    };
}

pub(crate) use decimal_operation_panics;

macro_rules! decimal_inexact {
    ($op: literal) => {
        concat!(
            "\n\n# Precision\n\n",
            "Since the result of ",
            $op,
            " is irrational number, it can usually only be computed to some finite precision ",
            "from a series of increasingly accurate approximations.\n\n",
            "The result of this operation is mostly inexact and raises [`OP_INEXACT`](crate#signaling-flags-and-trap-enablers) signal.",
            "\n\n"
        )
    };
}

pub(crate) use decimal_inexact;

macro_rules! example_header {
    ($bits: literal $($sign: ident)?) => {
        concat!(
"
# Examples

Please note that this example is shared between decimal types.
Which explains why ", doc::link_type_str!($bits $($sign)?), " is used here.")
    }
}

pub(crate) use example_header;

macro_rules! doc_comment {
    {$bits: literal $($sign: ident)?, $($($desc: expr)+)?, $(#Panics $($panics: expr)+,)? $(#Also $($also: expr)+,)? $(#Examples $(($H: literal))? $([$(@ {$S: ident})? $($code: expr)+])+),+ } => {
            concat!(
                $($($desc, "\n"), +,)?
                $(
                    "\n# Panics:\n\n",
                    $($panics), +,
                )?
                doc::example_header!($bits $($sign)?),
                $(
                    $("\n## ", $H, "\n\n",)?
                    "

```
use fastnum::*;

",
                    $(
                        doc::doc_comment!(@ {$($S)?} [$($code)+]),
                    )+
                    "```\n",
                )+
                $("\n", "See also: \n" $(, "- ", $also, "\n")+)?
            )
        };
    (@ {} [$($code: expr)+]) => {
        concat!($($code), +, "\n")
    };
    (@ {U} [$($code: expr)+]) => {
        ""
    };
}

pub(crate) use doc_comment;
