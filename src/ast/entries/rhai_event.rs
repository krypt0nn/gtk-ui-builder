use crate::ast::entry::Entry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RhaiEvent {
    pub name: String,
    pub code: String,
    event_id: usize
}

impl RhaiEvent {
    pub fn entry(name: String, code: String) -> Entry {
        Entry::RhaiEvent(Self {
            name,
            code,
            event_id: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as usize
        })
    }

    pub fn get_signal_name(&self) -> String {
        format!("rhai_signal_{}", self.event_id)
    }

    /// Get pretty string description of this entry
    pub fn dbg(&self) -> String {
        format!("{} => RhaiEvent({})", self.name, self.event_id)
    }

    /// Get XML description of this entry
    pub fn get_xml(&self) -> String {
        // format!("<signal name=\"{}\" handler=\"{}\"/>", self.name, self.get_signal_name())
        String::new()
    }
}
