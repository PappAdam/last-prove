use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use super::aligned_array::NoneValue;

#[derive(Debug)]
pub struct ObjVec<T: NoneValue> {
    first_empty_index: usize,
    count: usize,
    pub content: Vec<T>,
}

impl<T: NoneValue> ObjVec<T> {
    pub fn new() -> Self {
        Self {
            first_empty_index: usize::MAX,
            content: Vec::new(),
            count: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            first_empty_index: usize::MAX,
            count: 0,
            content: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: T) -> usize {
        let index_pushed;
        if self.first_empty_index != usize::MAX {
            self.content[self.first_empty_index] = value;
            index_pushed = self.first_empty_index;
            self.seek_for_empty();
        } else {
            index_pushed = self.count();
            self.content.push(value);
        }

        self.count += 1;

        index_pushed
    }

    pub fn remove(&mut self, index: usize) {
        self.content[index].set_to_none();
        if index < self.first_empty_index {
            self.first_empty_index = index;
        }

        self.count -= 1;
    }

    fn seek_for_empty(&mut self) {
        let mut index = usize::MAX;
        self.content.iter().enumerate().for_each(|(i, t)| {
            if t.is_none() == true {
                index = i;
                return;
            }
        });

        self.first_empty_index = index;
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl<T: NoneValue> Index<usize> for ObjVec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl<T: NoneValue> IndexMut<usize> for ObjVec<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[index]
    }
}

impl<'a, T: NoneValue> IntoIterator for &'a ObjVec<T> {
    type Item = &'a T;

    type IntoIter = ObjVecIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ObjVecIterator {
            vector: &self.content,
            index: 0,
        }
    }
}

pub struct ObjVecIterator<'a, T: NoneValue> {
    vector: &'a Vec<T>,
    index: usize,
}

impl<'a, T: NoneValue> Iterator for ObjVecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = None;
        if self.vector.len() > self.index {
            next = Some(&self.vector[self.index]);
            self.index += 1;
        }
        next
    }
}
