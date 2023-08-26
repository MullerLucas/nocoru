pub struct HellHandle {
    value: usize,
}

impl HellHandle {
    pub fn new(value: usize) -> Self {
        Self {
            value
        }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

impl From<usize> for HellHandle {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<HellHandle> for usize {
    fn from(value: HellHandle) -> Self {
        value.value()
    }
}
