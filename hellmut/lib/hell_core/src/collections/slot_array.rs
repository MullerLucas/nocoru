use crate::error::{HellResult, HellErrorHelper};

use super::hell_handle::HellHandle;

#[derive(Debug)]
pub struct SlotArray<T, const SIZE: usize> {
    data: [T; SIZE],
    is_free: [bool; SIZE],
}

impl<T, const SIZE: usize> SlotArray<T, SIZE> {
    fn find_first_free_slot(&self) -> Option<usize> {
        for idx in 0..SIZE {
            if self.is_free[idx] {
                return Some(idx);
            }
        }

        None
    }

    pub fn has_free_slots(&self) -> bool {
        self.find_first_free_slot().is_some()
    }

    pub fn try_push(&mut self, value: T) -> HellResult<HellHandle> {
        let Some(idx) = self.find_first_free_slot() else {
            return Err(HellErrorHelper::add_to_full_msg_err("slot array is full"));
        };

        self.data[idx] = value;
        self.is_free[idx] = false;

        Ok(idx.into())
    }

    pub fn push(&mut self, value: T) -> HellHandle {
        self.try_push(value).expect("failed to push into slot array")
    }

    pub fn remove(&mut self, handle: HellHandle) {
        debug_assert!(!self.is_free[handle.value()]);
        self.is_free[handle.value()] = true;
    }

    pub fn get(&self, handle: HellHandle) -> &T {
        debug_assert!(!self.is_free[handle.value()]);
        &self.data[handle.value()]
    }

    pub fn get_mut(&mut self, handle: HellHandle) -> &mut T {
        debug_assert!(!self.is_free[handle.value()]);
        &mut self.data[handle.value()]
    }
}
