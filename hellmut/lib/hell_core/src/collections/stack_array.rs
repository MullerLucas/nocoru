use std::array;
use std::ops::{Deref, DerefMut, IndexMut};
use std::slice::SliceIndex;
use std::{fmt, ops::Index};
use std::mem::MaybeUninit;

use crate::error::{HellResult, HellErrorHelper};

pub struct StackArray<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len:  usize,
}

impl<T, const N: usize> StackArray<T, N> {
    #[inline]
    pub fn new() -> Self {
        Self {
            // SAFETY: safe because we `assume_init` on an array with elements of type `MaybeUninit`
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len:  0,
        }
    }

    #[inline]
    pub fn from_fn(cb: impl FnMut(usize) -> MaybeUninit<T>) -> Self {
        Self {
            data: array::from_fn(cb),
            len: 0
        }
    }

    #[inline]
    pub fn from_defaults() -> Self
        where T: Default,
    {
        Self::from_fn(|_| MaybeUninit::new(Default::default()))
    }


    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    pub fn push(&mut self, value: T) {
        debug_assert!(!self.is_full());

        self.data[self.len].write(value);
        self.len += 1;
    }

    pub fn try_push(&mut self, value: T) -> HellResult<()> {
        if self.is_full() {
            return Err(HellErrorHelper::add_to_full_msg_err("trying to push into full StackArray"));
        }

        self.data[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> T {
        debug_assert!(!self.is_empty());

        self.len -= 1;
        let mut value = MaybeUninit::uninit();
        std::mem::swap(&mut value, &mut self.data[self.len]);
        unsafe { value.assume_init() }
    }

    pub fn try_pop(&mut self) -> HellResult<T> {
        if self.is_empty() {
            return Err(HellErrorHelper::remove_from_empty_msg_err("trying remove from empty StackArray"));
        }

        self.len -= 1;
        let mut value = MaybeUninit::uninit();
        std::mem::swap(&mut value, &mut self.data[self.len]);
        Ok(unsafe { value.assume_init() })
    }

    #[inline]
    pub fn extend_from_slice(&mut self, value: &[T])
        where T: Clone
    {
        debug_assert!(self.len + value.len() <= N);
        for (idx, val) in value.iter().cloned().enumerate() {
            self.data[idx + self.len].write(val);
        }

        self.len += value.len();
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            std::mem::transmute::<_, &[T]>(
                &self.data[0..self.len]
            )
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::mem::transmute::<_, &mut [T]>(
                &mut self.data[0..self.len]
            )
        }
    }
}

impl<T, const N: usize> Default for StackArray<T, N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, I, const N: usize> Index<I> for StackArray<T, N>
    where I: SliceIndex<[MaybeUninit<T>], Output = MaybeUninit<T>>
{
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        // SAFETY: data will be initialized for valid indices
        unsafe {
            self.data.index(index).assume_init_ref()
        }
    }
}

impl<T, I, const N: usize> IndexMut<I> for StackArray<T, N>
    where I: SliceIndex<[MaybeUninit<T>], Output = MaybeUninit<T>>
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        // SAFETY: data will be initialized for valid indices
        unsafe {
            self.data.index_mut(index).assume_init_mut()
        }
    }

}

impl<T: Clone, const N: usize> From<&[T]> for StackArray<T, N> {
    #[inline]
    fn from(value: &[T]) -> Self {
        let mut result = Self::new();
        result.extend_from_slice(value);
        result
    }
}

impl<T, const N: usize> AsRef<[T]> for StackArray<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> AsMut<[T]> for StackArray<T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> Deref for StackArray<T, N> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> DerefMut for StackArray<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

// TODO(lm): remove trailing ', '
impl<T, const N: usize> fmt::Debug for StackArray<T, N>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "len: '{:?}', data: [", self.len)?;

        for idx in 0..self.len {
            unsafe { write!(f, "{:?}, ", self.data[idx].assume_init_ref())?; }
        }

        write!(f, "]")?;
        Ok(())
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut arr = StackArray::<i32, 3>::default();
        assert_eq!(arr.len(), 0);

        arr.push(1);
        assert_eq!(arr.len(), 1);
        arr.push(2);
        assert_eq!(arr.len(), 2);

        assert!(arr.try_push(3).is_ok());
        assert!(arr.try_push(4).is_err());
    }

    #[test]
    fn test_pop() {
        let mut arr = StackArray::<i32, 3>::default();

        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert_eq!(arr.len(), 3);

        arr.pop();
        assert_eq!(arr.len(), 2);
        arr.pop();
        assert_eq!(arr.len(), 1);

        assert!(arr.try_pop().is_ok());
        assert!(arr.try_pop().is_err());
    }
}
