use crate::GameObject;

#[derive(Debug, PartialEq)]
pub enum ObjectTag {
    Empty,
    Map,

    //Index in map's structure ObjectVector
    Structure(usize),
}

impl<'a> GameObject<'a> {
    ///Checks if an object has a certain tag.
    pub fn has_tag(&self, tag: &ObjectTag) -> bool {
        self.tags.contains(&tag)
    }
    ///Tries to add a tag to an object. If the tag is already present, no change will be made.
    ///Returns true if tag was added, false if nothing changed.
    pub fn add_tag(&mut self, tag: ObjectTag) -> bool {
        if !self.has_tag(&tag) {
            self.tags.push(tag);
            return true;
        }
        false
    }
    ///Forces a tag on an object, even if the tag is present.
    ///DO NOT USE UNLESS COMPLETELY NESCESSARY
    pub fn force_tag(&mut self, tag: ObjectTag) {
        self.tags.push(tag);
    }

    ///Tries to remove a tag from an object.
    ///Returns false if tag wasn't present, true if the tag was removed.
    ///Only first occurance is removed, unexpected behavior can happen if you've used force_tag() before.
    pub fn remove_tag(&mut self, tag: &ObjectTag) -> bool {
        for (i, object_tag) in self.tags.iter().enumerate() {
            if tag == object_tag {
                self.tags.swap_remove(i);
                return true;
            }
        }
        false
    }
}
