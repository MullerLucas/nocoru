use std::{mem::MaybeUninit, fmt};
use crate::error::{HellResult, HellErrorHelper};



pub struct QueueArray<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    head_tail: Option<(usize, usize)>,
}

impl<T, const N: usize> QueueArray<T, N> {
    #[inline]
    pub fn new() -> Self {
        Self {
            // SAFETY: safe because we `assume_init` on an array with elements of type `MaybeUninit`
            data: unsafe { MaybeUninit::uninit().assume_init() },
            head_tail: None,
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        N
    }

    #[inline]
    pub fn len(&self) -> usize {
        let Some((head, tail)) = self.head_tail else {
            return 0;
        };

        if tail >= head {
            tail - head + 1
        } else {
            N - (head - tail - 1)
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head_tail.is_none()
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == N
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr() as _
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr() as _
    }

    pub fn try_enqueue(&mut self, value: T) -> HellResult<usize> {
        if self.is_full() {
            return Err(HellErrorHelper::add_to_full_msg_err("Queue is full"));
        }

        let (head, tail) = if let Some((head, mut tail)) = self.head_tail {
            tail += 1;
            if tail == N {
                tail = 0
            }
            (head, tail)
        } else {
            (0, 0)
        };

        let _ = self.data[tail].write(value);
        self.head_tail = Some((head, tail));

        Ok(tail)
    }

    pub fn try_dequeue(&mut self) -> HellResult<T> {
        let len = self.len();
        if len == 0 {
            return Err(HellErrorHelper::add_to_full_msg_err("Queue is full"));
        }
        // since len > 0 `head_tail` has to be `Some`
        let (mut head, tail) = self.head_tail.unwrap();

        let mut value =  MaybeUninit::uninit();
        std::mem::swap(&mut self.data[head], &mut value);
        let value = unsafe { value.assume_init() };

        if head == N - 1 {
            head = 0;
        } else {
            head += 1;
        }

        // if it contained 1 before, it's empty now
        if len == 1 {
            self.head_tail = None;
        } else {
            self.head_tail = Some((head, tail));
        }

        Ok(value)
    }

    pub fn head(&self) -> Option<&T> {
        let Some((head, _)) = self.head_tail else {
            return None;
        };

        // SAFETY: if head is set, it points to an initialized value
        unsafe { Some(self.data[head].assume_init_ref()) }
    }

    pub fn tail(&self) -> Option<&T> {
        let Some((_, tail)) = self.head_tail else {
            return None;
        };

        // SAFETY: if head is set, it points to an initialized value
        unsafe { Some(self.data[tail].assume_init_ref()) }
    }
}

impl<T, const N: usize> Default for QueueArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO(lm): remove trailing ', '
impl<T, const N: usize> fmt::Debug for QueueArray<T, N>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "head_tail: '{:?}', data: [", self.head_tail)?;

        if let Some((head, tail)) = self.head_tail {
            if tail > head {
                for idx in head..=tail {
                    unsafe { write!(f, "{:?}, ", self.data[idx].assume_init_ref())?; }
                }
            } else {
                for idx in head..N {
                    unsafe { write!(f, "{:?}, ", self.data[idx].assume_init_ref())?; }
                }
                for idx in 0..=tail {
                    unsafe { write!(f, "{:?}, ", self.data[idx].assume_init_ref())?; }
                }
            }
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
    fn test_enqueue() -> HellResult<()> {
        let mut arr = QueueArray::<i32, 5>::default();
        assert_eq!(arr.len(), 0);
        arr.try_enqueue(1)?;
        assert_eq!(arr.len(), 1);
        arr.try_enqueue(2)?;
        assert_eq!(arr.len(), 2);
        arr.try_enqueue(3)?;
        arr.try_enqueue(4)?;
        arr.try_enqueue(5)?;
        assert!(arr.try_enqueue(6).is_err());
        Ok(())
    }

    #[test]
    fn test_dequeue() -> HellResult<()> {
        let mut arr = QueueArray::<i32, 3>::default();
        arr.try_enqueue(1)?;
        arr.try_enqueue(2)?;
        arr.try_enqueue(3)?;
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.try_dequeue()?, 1);
        assert_eq!(arr.try_dequeue()?, 2);
        assert_eq!(arr.try_dequeue()?, 3);
        Ok(())
    }

    #[test]
    fn test_cycle() -> HellResult<()> {
        let mut arr = QueueArray::<i32, 3>::default();
        arr.try_enqueue(1)?;
        arr.try_enqueue(2)?;
        arr.try_enqueue(3)?;
        assert_eq!(arr.len(), 3);

        assert_eq!(arr.try_dequeue()?, 1);
        arr.try_enqueue(4)?;
        assert_eq!(arr.try_dequeue()?, 2);
        arr.try_enqueue(5)?;
        assert_eq!(arr.try_dequeue()?, 3);

        assert_eq!(arr.try_dequeue()?, 4);
        assert_eq!(arr.try_dequeue()?, 5);
        assert!   (arr.try_dequeue().is_err());

        assert_eq!(arr.len(), 0);
        Ok(())
    }
}
