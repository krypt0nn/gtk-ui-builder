use crate::ast::entry::Entry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    pub class: String,
    pub name: Option<String>,
    pub children: Vec<Entry>
}

impl Object {
    pub fn entry(class: String, name: Option<String>, children: Vec<Entry>) -> Entry {
        Entry::Object(Self { class, name, children })
    }

    pub fn add_child(&mut self, child: Entry) {
        self.children.push(child);
    }

    /// Get pretty string description of this entry
    pub fn dbg(&self) -> String {
        format!(
            "Object {{\n  class: {},\n  name: {:?},\n  children: [\n{}  ]\n}}",
            self.class,
            self.name,
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
        let class = self.class.replace(".", "");

        let beginning = match &self.name {
            Some(name) => format!("<object class=\"{}\" id=\"{}\">", class, name),
            None => format!("<object class=\"{}\">", class)
        };

        let mut signals = String::new();
        let mut properties = String::new();
        let mut children = String::new();

        for child in &self.children {
            #[cfg(feature = "rhai-events")]
            if let Entry::RhaiEvent(event) = child {
                signals += &event.get_xml();

                continue;
            }
            
            if let Entry::Property(property) = child {
                properties += &property.get_xml();
            }

            else {
                children += &format!("<child>{}</child>", child.get_xml());
            }
        }

        format!("{}{}{}{}</object>", beginning, signals, properties, children)
    }
}
