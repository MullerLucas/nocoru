use std::{array, fmt::{Debug, Display, Pointer}};


pub struct StackArray<T, const SIZE: usize> {
    data: [T; SIZE],
    len: usize,
}

impl<T, const SIZE: usize> Default for StackArray<T, SIZE>
    where T: Default,
{
    fn default() -> Self {
        Self::from_fn(|_| T::default())
    }
}

impl<T, const SIZE: usize> StackArray<T, SIZE>
    where [T; SIZE]: Default,
{
    pub fn from_default_array() -> Self {
        let data: [T; SIZE] = Default::default();
        Self::new(data, 0)
    }
}

impl<T, const SIZE: usize> Into<Vec<T>> for StackArray<T, SIZE>
    where T: Default + Clone
{
    fn into(self) -> Vec<T> {
        self.to_vec()
    }
}

impl<T, const SIZE: usize> From<&[T]> for StackArray<T, SIZE>
    where T: Default + Clone
{
    fn from(val: &[T]) -> Self {
        let mut res = Self::from_fn(|idx| {
            if val.len() > idx {
                val[idx].clone()
            } else {
                T::default()
            }
        });
        res.len = val.len().min(SIZE);
        res
    }
}

impl<T, const SIZE: usize> StackArray<T, SIZE> {
    pub fn new(data: [T; SIZE], len: usize) -> Self {
        debug_assert!(len <= SIZE);

        Self {
            data,
            len,
        }
    }
    pub fn from_fn(cb: impl FnMut(usize) -> T) -> Self {
        let data = array::from_fn(cb);
        Self::new(data, 0)
    }

    // ------------------------------------------------------------------------

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == SIZE
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[0..self.len()]
    }

    pub fn push(&mut self, val: T) {
        if self.is_full() {
            panic!("trying to push into a full DynArray");
        }

        self.data[self.len] = val;
        self.len += 1;
    }

    pub fn set(&mut self, idx: usize, val: T) {
        if idx >= self.len() {
            panic!("trying to set invalid index '{}'", idx);
        }

        self.data[idx] = val;
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.data.get_mut(idx)
    }

    pub fn to_vec(self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len());
        for data in self.data { v.push(data); }
        v
    }
}

impl<T, const SIZE: usize> std::ops::Index<usize> for StackArray<T, SIZE> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<T, const SIZE: usize> std::ops::IndexMut<usize> for StackArray<T, SIZE> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<T, const SIZE: usize> Display for StackArray<T, SIZE>
    where T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T, const SIZE: usize> Debug for StackArray<T, SIZE>
    where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T, const SIZE: usize> Clone for StackArray<T, SIZE>
    where T: Clone
{
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), len: self.len.clone() }
    }
}
