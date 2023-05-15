use std::{
    alloc::{alloc, dealloc, Layout},
    fmt::Display,
    mem::size_of,
    ops::{Index, IndexMut},
    ptr::write_bytes,
};

use crate::resources::buffer::DynamicUniformBuffer;

pub trait AlignedValue {
    fn is_none(&self) -> bool;
}

pub struct AlignedArray<T> {
    length: usize,
    data: *mut T,
    layout: Option<Layout>,
    aligned_data_size: usize,
    first_none_index: Option<usize>,
}

impl<T: AlignedValue> AlignedArray<T> {
    pub fn from_alignment(alignment: usize, size: usize) -> Result<Self, String> {
        let allocation_layout = Layout::from_size_align(size * size_of::<T>(), alignment)
            .expect("Failed create layout");
        let data = unsafe { alloc(allocation_layout) };

        if data.is_null() {
            return Err(String::from("Failed to allocate memory"));
        }

        let aligned_data_size =
            (size_of::<T>() as f32 / alignment as f32).ceil() as usize * size_of::<T>();

        Ok(Self {
            length: size,
            data: data as *mut _,
            layout: Some(allocation_layout),
            aligned_data_size,
            first_none_index: Some(0),
        })
    }

    pub fn from_dynamic_ub_data(dynamic_ub: &DynamicUniformBuffer) -> Self {
        unsafe {
            write_bytes(
                dynamic_ub.buffer_pointer,
                0u8,
                dynamic_ub.size as usize * dynamic_ub.alignment,
            );
        }

        Self {
            length: dynamic_ub.size as _,
            data: dynamic_ub.buffer_pointer as _,
            layout: None,
            aligned_data_size: dynamic_ub.alignment,
            first_none_index: Some(0),
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.data
    }

    /// Insert value to the first possible index
    /// - Returns the index, the data was pushed into
    pub fn push(&mut self, value: T) -> Result<usize, ()> {
        let index = match self.first_none_index {
            Some(index) => {
                self[index] = value;
                index
            }
            None => return Err(()),
        };

        let mut i = index;
        while i < self.length && !(self[i].is_none()) {
            i += 1;
        }

        self.first_none_index = if i < self.length { Some(i) } else { None };

        return Ok(index);
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn get_data_pointer(&self, index: usize) -> *mut T {
        (self.data as usize + self.aligned_data_size * index) as *mut T
    }
}

impl<T> Index<usize> for AlignedArray<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*((self.data as usize + self.aligned_data_size * index) as *const T) }
    }
}

impl<T> IndexMut<usize> for AlignedArray<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *((self.data as usize + self.aligned_data_size * index) as *mut T) }
    }
}

// impl<T: Copy + Clone> Iterator for AlignedArray<T> {
//     type Item = T;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.length == 0 {
//             return None;
//         }

//         self.data = (self.data as usize + self.aligned_data_size) as *mut _;
//         self.length -= 1;

//         return Some(self[0]);
//     }
// }

impl<T> Drop for AlignedArray<T> {
    fn drop(&mut self) {
        unsafe {
            if let Some(layout) = self.layout {
                dealloc(self.data as _, layout)
            }
        }
    }
}
