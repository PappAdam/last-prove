pub mod aligned_array;
pub mod aligned_array_implementations;

use std::{mem::size_of, ptr::copy_nonoverlapping};

use nalgebra::SVector;
use num::{traits::AsPrimitive, Num};

#[macro_export]
macro_rules! offset_of {
    ($type:ty, $field:ident) => {{
        unsafe { &(*(0 as usize as *const $type)).$field as *const _ as usize }
    }};
}

pub trait Convert<T> {
    fn conv(self) -> T;
}

impl<T: Num + AsPrimitive<U>, const R: usize, U: Num + Copy + 'static, const Y: usize>
    Convert<SVector<U, Y>> for SVector<T, R>
{
    fn conv(self) -> SVector<U, Y> {
        unsafe {
            let raw_data_t = [T::zero(); Y];
            let mut raw_data_u = [U::zero(); Y];
            copy_nonoverlapping(
                &self as *const _ as *const u8,
                &raw_data_t as *const _ as *mut _,
                R.min(Y) * size_of::<T>(),
            );

            for (i, coord) in raw_data_t.iter().enumerate() {
                raw_data_u[i] = coord.as_();
            }

            *(&raw_data_u as *const _ as *const SVector<U, Y>)
        }
    }
}
