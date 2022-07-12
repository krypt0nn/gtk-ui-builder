use crate::ast::entry::Entry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Requirement {
    Gtk(String),
    Libadwaita(String),
    Other(String, String)
}

impl Requirement {
    pub fn new<T: ToString>(lib: T, version: T) -> Self {
        let version = version.to_string();

        match lib.to_string().as_str() {
            "gtk" => Self::Gtk(version),
            "Gtk" => Self::Gtk(version),
            "GTK" => Self::Gtk(version),

            "adw" => Self::Libadwaita(version),
            "Adw" => Self::Libadwaita(version),
            "ADW" => Self::Libadwaita(version),
            "libadwaita" => Self::Libadwaita(version),
            "Libadwaita" => Self::Libadwaita(version),

            lib => Self::Other(lib.to_string(), version)
        }
    }

    pub fn get_xml(&self) -> Option<String> {
        let (lib, version) = match self {
            Requirement::Gtk(version) => ("gtk", version),
            Requirement::Other(lib, version) => (lib.as_str(), version),

            Requirement::Libadwaita(_) => return None // ("libadwaita", version)
        };

        Some(format!("<requires lib=\"{}\" version=\"{}\"/>", lib, version))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Root {
    pub requirements: Vec<Requirement>,
    pub children: Vec<Entry>
}

impl Root {
    pub fn add_child(&mut self, child: Entry) {
        self.children.push(child);
    }

    pub fn require(&mut self, requirement: Requirement) {
        self.requirements.push(requirement);
    }

    /// Get pretty string description of this entry
    pub fn dbg(&self) -> String {
        format!(
            "Root {{\n  requirements: {:?},\n  children: [\n{}  ]\n}}",
            self.requirements,
            (&self.children).into_iter().map(|child| {
                let text = child.dbg().lines()
                    .map(|line| String::from("      ") + line + "\n")
                    .collect::<String>();

                text.trim_end().to_string() + ",\n"
            }).collect::<String>()
        )
    }

    /// Get XML description of this entry
    pub fn get_xml(&self) -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><interface>{}{}</interface>",
            (&self.requirements).into_iter().map(|item| item.get_xml()).filter_map(|item| item).collect::<String>(),
            (&self.children).into_iter().map(|item| item.get_xml()).collect::<String>()
        )
    }

    pub fn filter<T: Fn(&Entry) -> Option<F>, F>(&self, filter: &T) -> Vec<F> {
        fn filter_entries<T: Fn(&Entry) -> Option<F>, F>(entries: &Vec<Entry>, filter: &T) -> Vec<F> {
            let mut filtered = Vec::new();

            for entry in entries {
                if let Some(found) = filter(entry) {
                    filtered.push(found);
                }

                match entry {
                    Entry::Object(obj) => filtered.append(&mut filter_entries(&obj.children, filter)),
                    Entry::Property(prop) => {
                        if let super::property::PropertyValue::Entry(obj) = &prop.value {
                            if let Some(found) = filter(&Entry::Object(obj.clone())) {
                                filtered.push(found);
                            }

                            filtered.append(&mut filter_entries(&obj.children, filter));
                        }
                    }
                    _ => ()
                }
            }

            filtered
        }

        filter_entries(&self.children, filter)
    }

    pub fn get_named_objects(&self) -> Vec<(String, super::object::Object)> {
        self.filter(&|entry| {
            match entry {
                Entry::Object(obj) => {
                    if let Some(name) = &obj.name {
                        Some((name.clone(), obj.clone()))
                    }

                    else {
                        None
                    }
                },
                _ => None
            }
        })
    }

    #[cfg(feature = "rhai-events")]
    pub fn get_rhai_events(&self) -> Vec<super::rhai_event::RhaiEvent> {
        self.filter(&|entry| {
            match entry {
                Entry::RhaiEvent(event) => Some(event.clone()),
                _ => None
            }
        })
    }
}
