use super::GameObject;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct ObjVec<T: GameObject> {
    first_empty_index: usize,
    pub content: Vec<T>,
}

impl<T: GameObject + Debug> ObjVec<T> {
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

    pub fn push(&mut self, value: T) -> u16 {
        let index_pushed;
        if self.first_empty_index != usize::MAX {
            self.content[self.first_empty_index] = value;
            index_pushed = self.first_empty_index as u16;
            self.seek_for_empty();
        } else {
            index_pushed = self.len() as u16;
            self.content.push(value);
        }
        index_pushed
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

    pub fn len(&self) -> usize {
        let mut len = 0;
        for element in &self.content {
            if !element.is_none() {
                len += 1
            }
        }
        len
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

impl<'a, T: GameObject> IntoIterator for &'a ObjVec<T> {
    type Item = &'a T;

    type IntoIter = ObjVecIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ObjVecIterator {
            vector: &self.content,
            index: 0,
        }
    }
}

pub struct ObjVecIterator<'a, T: GameObject> {
    vector: &'a Vec<T>,
    index: usize,
}

impl<'a, T: GameObject> Iterator for ObjVecIterator<'a, T> {
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
