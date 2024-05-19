use std::collections::LinkedList;

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

pub trait LinkedListExt<T> {
    fn get(&self, idx: usize) -> Option<&T>;
    fn get_owned(self, idx: usize) -> Option<T>;
    fn get_rev(&self, idx: usize) -> Option<&T>;
    fn get_rev_owned(self, idx: usize) -> Option<T>;
}

impl<T> LinkedListExt<T> for LinkedList<T> {
    fn get(&self, idx: usize) -> Option<&T> {
        self.iter().nth(idx)
    }

    fn get_owned(self, idx: usize) -> Option<T> {
        self.into_iter().nth(idx)
    }

    fn get_rev(&self, idx: usize) -> Option<&T> {
        self.iter().rev().nth(idx)
    }

    fn get_rev_owned(self, idx: usize) -> Option<T> {
        self.into_iter().rev().nth(idx)
    }
}
