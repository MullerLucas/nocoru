use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ResourceHandle {
    pub idx: usize,
}

impl fmt::Display for ResourceHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.idx.fmt(f)
    }
}

impl ResourceHandle {
    pub const INVALID: ResourceHandle = Self::new(usize::MAX);

    pub const fn new(idx: usize) -> Self {
        Self {
            idx
        }
    }
}
