use super::entries::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry {
    Root(Root),
    Object(Object),
    Property(Property),

    #[cfg(feature = "rhai-events")]
    RhaiEvent(RhaiEvent)
}

impl Entry {
    pub fn dbg(&self) -> String {
        match self {
            Self::Root(obj) => obj.dbg(),
            Self::Object(obj) => obj.dbg(),
            Self::Property(obj) => obj.dbg(),

            #[cfg(feature = "rhai-events")]
            Self::RhaiEvent(obj) => obj.dbg()
        }
    }

    pub fn get_xml(&self) -> String {
        match self {
            Self::Root(obj) => obj.get_xml(),
            Self::Object(obj) => obj.get_xml(),
            Self::Property(obj) => obj.get_xml(),

            #[cfg(feature = "rhai-events")]
            Self::RhaiEvent(obj) => obj.get_xml()
        }
    }
}
