use super::entry::Entry;
use super::entries::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct Tree {
    pub root: Root
}

impl Tree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_child(&mut self, child: Entry) {
        self.root.add_child(child);
    }

    pub fn require(&mut self, requirement: Requirement) {
        self.root.require(requirement);
    }

    /// Get XML description of the tree
    pub fn get_xml(&self) -> String {
        self.root.get_xml()
    }
}
