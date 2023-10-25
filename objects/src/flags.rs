use std::fmt::Debug;

use derive::Flag;

pub trait Flag {
    fn into_usize(&self) -> usize;
}

#[derive(Flag, Clone, Copy)]
pub enum GameObjectFlag {
    None = 0,
    NotClickable,
    Test1,
    Test2,
    Test3,
}

#[repr(C)]
pub struct Flags<T: Flag> {
    data: T,
}

impl<T: Flag> Flags<T> {
    pub fn has_flag(&self, flag: T) -> bool {
        unsafe {
            GameObjectFlag;
            let flag = flag.into_usize();
            let mut _data_ptr = (&self.data as *const _ as usize + flag / 8 as usize) as *mut u8;
            let element = flag % 8;
            return *_data_ptr & (1 << element) == (1 << element);
        }
    }

    pub fn activate_flag(&mut self, flag: T) {
        unsafe {
            let flag = flag.into_usize();
            let mut _data_ptr = (&self.data as *const _ as usize + flag / 8 as usize) as *mut u8;
            *_data_ptr = *_data_ptr | (1 << (flag % 8));
        }
    }

    pub fn deactivate_flag(&mut self, flag: T) {
        unsafe {
            let flag = flag.into_usize();
            let mut _data_ptr = (&self.data as *const _ as usize + flag / 8 as usize) as *mut u8;
            *_data_ptr = *_data_ptr & !(1 << (flag % 8));
        }
    }
}

impl<T: Flag> Debug for Flags<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "{:b}", *(&self.data as *const _ as *const u8)) }
    }
}

// #[derive(Default, Debug, PartialEq, Clone, Copy)]
// pub enum ObjectTag {
//     #[default]
//     Empty,
//     Map,

//     //Index in map's structure ObjectVector
//     Structure(usize),
// }

// impl<'a> GameObject<'a> {
//     ///Checks if an object has a certain tag.
//     pub fn has_tag(&self, tag: &ObjectTag) -> bool {
//         self.tags.contains(&tag)
//     }
//     ///Tries to add a tag to an object. If the tag is already present, no change will be made.
//     ///Returns true if tag was added, false if nothing changed.
//     pub fn add_tag(&mut self, tag: ObjectTag) -> bool {
//         if !self.has_tag(&tag) {
//             self.tags.push(tag);
//             return true;
//         }
//         false
//     }
//     ///Forces a tag on an object, even if the tag is present.
//     ///DO NOT USE UNLESS COMPLETELY NESCESSARY
//     pub fn force_tag(&mut self, tag: ObjectTag) {
//         self.tags.push(tag);
//     }

//     ///Tries to remove a tag from an object.
//     ///Returns false if tag wasn't present, true if the tag was removed.
//     ///Only first occurance is removed, unexpected behavior can happen if you've used force_tag() before.
//     pub fn remove_tag(&mut self, tag: &ObjectTag) -> bool {
//         for (i, object_tag) in self.tags.iter().enumerate() {
//             if tag == object_tag {
//                 self.tags.swap_remove(i);
//                 return true;
//             }
//         }
//         false
//     }
// }

#[cfg(test)]
mod tests {
    use crate::flags::Flags;
    use super::Flag;

    #[derive(Default, Clone, Copy, Flag)]
    enum TestFlag {
        #[default]
        Test0 = 0,
        Test1 = 1,
        Test2 = 2,
        Test3 = 3,
        Test4 = 4,
        Test5 = 5,
        Test6 = 6,
        Test7 = 7,
        Test8 = 8,
        Test9 = 9,
        Test10 = 10,
        Test11 = 11,
    }

    #[test]
    fn test() {
        let flag_1 = TestFlag::Test0;
        // let mut testflag = Flags {
        //     data: TestFlag::default(),
        // };
        // testflag.activate_flag(TestFlag::Test1);
        // testflag.activate_flag(TestFlag::Test11);
        // assert!(testflag.has_flag(TestFlag::Test1));
        // testflag.deactivate_flag(TestFlag::Test1);
        // assert!(!testflag.has_flag(TestFlag::Test1));
        // assert!(!testflag.has_flag(TestFlag::Test11));
    }
}
