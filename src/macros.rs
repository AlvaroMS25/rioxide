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

macro_rules! hashmap {
    ($($k: ident => $v: expr),*) => {{
        let mut hm = std::collections::HashMap::new();

        $(
            hm.insert(stringify!($k), $v);
        )*

        hm
    }};
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
            #[allow(unused)]
            $v fn from_str(item: &str) -> Option<Self> {
                Some(match item {
                    $($value => Self::$variant,)*
                    _ => return None,
                })
            }
        }
    };
}

/// Macro that generates getters and utility methods for all the variants of an enum
macro_rules! get_enum {
    (
        $(#[$outer_meta:meta])*
        $v: vis enum $name:ident $(<$($generics:tt),*>)? {
        $($(#[$inner_meta:meta])* $variant:ident($inner: ty)),* $(,)?
    }) => {
        $(#[$outer_meta])*
        $v enum $name $(<$($generics),*>)? {
            $(
                $(#[$inner_meta])*
                $variant($inner),
            )*
        }

        paste::paste!{
            impl $(<$($generics),*>)? $name $(<$($generics),*>)? {
                $(
                    #[allow(unused)]
                    $v fn [<is_ $variant:lower>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }

                    #[allow(unused)]
                    $v fn [<get_ $variant:lower>](&self) -> Option<&$inner> {
                        if self.[<is_ $variant:lower>]() {
                            Some(unsafe {self.[<get_ $variant:lower _unchecked>]()})
                        } else {
                            None
                        }
                    }

                    #[allow(unused)]
                    $v unsafe fn [<get_ $variant:lower _unchecked>](&self) -> &$inner {
                        match self {
                            Self::$variant(inner) => inner,
                            _ => unsafe { std::hint::unreachable_unchecked() }
                        }
                    }

                    #[allow(unused)]
                    $v fn [<get_ $variant:lower _mut>](&mut self) -> Option<&mut $inner> {
                        if self.[<is_ $variant:lower>]() {
                            Some(unsafe {self.[<get_ $variant:lower _unchecked_mut>]()})
                        } else {
                            None
                        }
                    }

                    #[allow(unused)]
                    $v unsafe fn [<get_ $variant:lower _unchecked_mut>](&mut self) -> &mut $inner {
                        match self {
                            Self::$variant(inner) => inner,
                            _ => unsafe { std::hint::unreachable_unchecked() }
                        }
                    }
                )*
            }
        }
    };
}

pub(crate) use mfn;
pub(crate) use swm;
pub(crate) use sw;
pub(crate) use c;
pub(crate) use enum_from_str;
pub(crate) use get_enum;
pub(crate) use hashmap;

fn testa() -> i32 {
    let itm = "Hello world";
    mfn!(itm, contains, match {
        "!" => {1},
        "H" => {4},
        else => { 453}
    });
}
