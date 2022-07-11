use crate::parser::prelude::*;
use crate::ast::entries::prelude::*;

#[test]
fn check_tokenizing_error() {
    let tree = Parser::parse("\"wrong string format");

    if let Err(ParseError::TokenizeError(_)) = tree { assert!(true); } else { assert!(false); }
}

#[test]
fn check_use_statement() {
    let tree = Parser::parse("using adw 1.0 using gtk 4.0");

    assert!(tree.is_ok());

    let tree = tree.unwrap();

    assert_eq!(tree.root.requirements[0], Requirement::Libadwaita(String::from("1.0")));
    assert_eq!(tree.root.requirements[1], Requirement::Gtk(String::from("4.0")));

    assert_eq!(tree.get_xml(), String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><interface><requires lib=\"gtk\" version=\"4.0\"/></interface>"));
}

#[test]
fn check_use_statement_error() {
    let tree = Parser::parse("using [] 1.0");

    if let Err(ParseError::IncorrectUseStatement { .. }) = tree {
        assert!(true);
    }
    
    else {
        assert!(false);
    }
}

#[test]
fn check_object_definition() {
    let tree = Parser::parse("Gtk.Button {}");

    assert!(tree.is_ok());
    assert_eq!(tree.unwrap().get_xml(), String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><interface><object class=\"GtkButton\"></object></interface>"));

    let tree = Parser::parse("Gtk.Button button_name {}");

    assert!(tree.is_ok());
    assert_eq!(tree.unwrap().get_xml(), String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><interface><object class=\"GtkButton\" id=\"button_name\"></object></interface>"));
}

#[test]
fn check_object_definition_error() {
    let tree = Parser::parse("Gtk.Button");

    if let Err(ParseError::IncorrectObjectDefinition { .. }) = tree { assert!(true); } else { assert!(false); }

    let tree = Parser::parse("Gtk.Button button_name");

    if let Err(ParseError::IncorrectObjectDefinition { .. }) = tree { assert!(true); } else { assert!(false); }

    let tree = Parser::parse("Gtk.Button button name");

    if let Err(ParseError::IncorrectObjectDefinition { .. }) = tree { assert!(true); } else { assert!(false); }
}

#[test]
fn check_properties() {
    let tree = Parser::parse("example-property: example-value;");

    assert!(tree.is_ok());
    assert_eq!(tree.unwrap().get_xml(), String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><interface><property name=\"example-property\">example-value</property></interface>"));

    let tree = Parser::parse("example-property: \"Example value\";");

    assert!(tree.is_ok());
    assert_eq!(tree.unwrap().get_xml(), String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><interface><property name=\"example-property\">Example value</property></interface>"));
}

#[test]
fn check_properties_error() {
    let tree = Parser::parse("example-property:");

    if let Err(ParseError::IncorrectPropertyDefinition { .. }) = tree { assert!(true); } else { assert!(false); }

    let tree = Parser::parse("example-property: example-value");

    if let Err(ParseError::IncorrectPropertyDefinition { .. }) = tree { assert!(true); } else { assert!(false); }

    let tree = Parser::parse("example-property: example-value ;");

    if let Err(ParseError::TokenizeError(_)) = tree { assert!(true); } else { assert!(false); }
}

#[test]
fn check_complex_parsing() {
    let tree = Parser::parse("
        using Gtk 4.0
        using Adw 1

        Adw.ApplicationWindow window {
            default-width: 600;
            default-height: 500;

            content: Gtk.Box {
                orientation: vertical;

                Adw.HeaderBar {
                    title-widget: Adw.WindowTitle {
                        title: \"Example app\";
                    };
                }

                Adw.PreferencesPage {
                    Adw.PreferencesGroup {
                        vexpand: true;
                        valign: center;

                        Gtk.Button {
                            label: \"Hello, World!\";
                        }
                    }
                }
            };
        }
    ");

    assert!(tree.is_ok());
    assert_eq!(tree.unwrap().get_xml(), String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><interface><requires lib=\"gtk\" version=\"4.0\"/><object class=\"AdwApplicationWindow\" id=\"window\"><property name=\"default-width\">600</property><property name=\"default-height\">500</property><property name=\"content\"><object class=\"GtkBox\"><property name=\"orientation\">vertical</property><child><object class=\"AdwHeaderBar\"><property name=\"title-widget\"><object class=\"AdwWindowTitle\"><property name=\"title\">Example app</property></object></property></object></child><child><object class=\"AdwPreferencesPage\"><child><object class=\"AdwPreferencesGroup\"><property name=\"vexpand\">true</property><property name=\"valign\">center</property><child><object class=\"GtkButton\"><property name=\"label\">Hello, World!</property></object></child></object></child></object></child></object></property></object></interface>"));
}
