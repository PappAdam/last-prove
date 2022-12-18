use std::ops::{Index, IndexMut};

pub trait GameObject {
    fn is_none(&self) -> bool;
    fn set_to_none(&mut self);
}

#[derive(Debug)]
pub struct ObjVec<T: GameObject> {
    first_empty_index: usize,
    content: Vec<T>,
}

impl<T: GameObject> ObjVec<T> {
    pub fn new() -> Self {
        Self {
            first_empty_index: usize::MAX,
            content: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            first_empty_index: usize::MAX,
            content: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: T) {
        if self.first_empty_index != usize::MAX {
            self.content[self.first_empty_index] = value;
            self.seek_for_empty();
        } else {
            self.content.push(value);
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.content[index].set_to_none();
        if index < self.first_empty_index {
            self.first_empty_index = index;
        }
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
}

impl<T: GameObject> Index<usize> for ObjVec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl<T: GameObject> IndexMut<usize> for ObjVec<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[index]
    }
}
