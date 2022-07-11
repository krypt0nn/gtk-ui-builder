use crate::ast::entry::Entry;
use crate::ast::entries::object::Object;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValue {
    Text(String),
    Entry(Object)
}

impl PropertyValue {
    pub fn dbg(&self) -> String {
        match self {
            PropertyValue::Text(text) => text.clone(),
            PropertyValue::Entry(entry) => entry.dbg()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue
}

impl Property {
    pub fn entry(name: String, value: PropertyValue) -> Entry {
        Entry::Property(Self { name, value })
    }

    /// Get pretty string description of this entry
    pub fn dbg(&self) -> String {
        format!(
            "Property {{\n  name: {},\n  value: {}\n}}",
            self.name,
            self.value.dbg().lines().map(|line| String::from("  ") + line + "\n").collect::<String>().trim()
        )
    }

    /// Get XML description of this entry
    pub fn get_xml(&self) -> String {
        format!("<property name=\"{}\">{}</property>", self.name, {
            match &self.value {
                PropertyValue::Text(text) => text.clone(),
                PropertyValue::Entry(entry) => entry.get_xml()
            }
        })
    }
}
