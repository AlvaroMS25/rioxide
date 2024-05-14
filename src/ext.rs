pub trait StrExt {
    fn make_static(&self) -> &'static Self;
}

impl StrExt for str {
    fn make_static(&self) -> &'static Self {
        Box::leak(self.to_string().into_boxed_str())
    }
}

pub trait OptionTupleExt<A1, A2>: Sized {
    fn untuple_none(self) -> Option<(A1, A2)>;
}

impl<A1, A2> OptionTupleExt<A1, A2> for (Option<A1>, Option<A2>) {
    fn untuple_none(self) -> Option<(A1, A2)> {
        Some((self.0?, self.1?))
    }
}
