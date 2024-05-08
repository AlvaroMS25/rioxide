pub trait StrExt {
    fn make_static(&self) -> &'static Self;
}

impl StrExt for str {
    fn make_static(&self) -> &'static Self {
        Box::leak(self.to_string().into_boxed_str())
    }
}
