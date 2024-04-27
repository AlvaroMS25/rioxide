macro_rules! sw {
    ($it: ident, $pat: literal, || $($tree:tt)*) => {
        if $it.starts_with($pat) {
            return { $($tree)* };
        }
    };
}

macro_rules! c {
    ($it: ident, $pat: literal, || $($tree:tt)*) => {
        if $it.contains($pat) {
            return { $($tree)* };
        }
    };
}

macro_rules! mfn {
    ($it: ident, $fun: ident, match {
        $($ex: expr => { $($tree: tt)* }),* $(,)? $(else => { $($else_tree: tt)* })?
    }) => {
        $(
            if $it.$fun($ex) {
                return { $($tree)* }
            }
        )*

        $(
            return { $($else_tree)* };
        )?
    };
}

macro_rules! swm {
    ($it: ident, $($tree:tt)*) => {
        crate::macros::mfn!($it, starts_with, $($tree)*);
    };
}

macro_rules! enum_from_str {
    ($v: vis enum $name:ident {
        $($(#[$inner_meta:meta])* $variant:ident = $value:literal),* $(,)?
    }) => {
        $v enum $name {
            $(
                $(#[$inner_meta])*
                $variant,
            )*
        }

        impl $name {
            $v fn from_str(item: &str) -> Option<Self> {
                Some(match item {
                    $($value => Self::$variant,)*
                    _ => return None,
                })
            }
        }
    };
}

pub(crate) use mfn;
pub(crate) use swm;
pub(crate) use sw;
pub(crate) use c;
pub(crate) use enum_from_str;

fn testa() -> i32 {
    let itm = "Hello world";
    mfn!(itm, contains, match {
        "!" => {1},
        "H" => {4},
        else => { 453}
    });
}
