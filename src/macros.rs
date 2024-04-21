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

pub(crate) use mfn;
pub(crate) use swm;
pub(crate) use sw;
pub(crate) use c;

fn testa() -> i32 {
    let itm = "Hello world";
    mfn!(itm, contains, match {
        "!" => {1},
        "H" => {4},
        else => { 453}
    });
}
