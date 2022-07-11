use crate::parser::parser::Parser;
use crate::parser::parse_error::ParseError;

pub struct Builder {
    pub builder: gtk4::Builder
}

impl Builder {
    pub fn new<T: ToString>(blueprint: T) -> Result<Self, ParseError> {
        let tree = Parser::parse(blueprint)?;
        let builder = gtk4::Builder::from_string(&tree.get_xml());

        Ok(Self {
            builder
        })
    }

    pub fn object<T: gtk4::glib::IsA<gtk4::glib::Object>>(&self, name: &str) -> Option<T> {
        self.builder.object(name)
    }
}
