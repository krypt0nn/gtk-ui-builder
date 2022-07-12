use gtk4::prelude::ObjectExt;

use crate::parser::parser::Parser;
use crate::parser::parse_error::ParseError;

pub struct Builder {
    pub builder: gtk4::Builder
}

impl Builder {
    pub fn new<T: ToString>(blueprint: T) -> Result<Self, ParseError> {
        let tree = Parser::parse(blueprint)?;
        let builder = gtk4::Builder::from_string(&tree.get_xml());

        // println!("{}", tree.root.dbg());

        #[cfg(feature = "rhai-events")]
        {
            let engine = rhai::Engine::new();

            let builder_copy = builder.clone();
            let named_objects = tree.root.get_named_objects().into_iter().map(move |entry| {
                (entry.0.clone(), builder_copy.object::<gtk4::glib::Object>(&entry.0).unwrap())
            }).collect::<Vec<(String, gtk4::glib::Object)>>();

            // Know no better way to do that
            for object in builder.objects() {
                for event in tree.root.get_rhai_events() {
                    let compiled = engine.compile(event.code).expect("Failed to compile rhai script");

                    let this = object.clone();
                    let named_objects_copy = named_objects.clone();

                    if let Err(_) = object.try_connect_local(&event.name, true, move |_| {
                        let mut scope = rhai::Scope::new();

                        for (name, obj) in &named_objects_copy {
                            scope.push(name, obj.clone());
                        }

                        scope.push("self", this.clone());

                        let result = rhai::Engine::new()
                            .register_type::<gtk4::glib::Object>()

                            .register_fn("get_str", |obj: &mut gtk4::glib::Object, prop: &str| -> String {
                                obj.property_value(prop).get().unwrap()
                            })
                            .register_fn("get_num", |obj: &mut gtk4::glib::Object, prop: &str| -> i64 {
                                obj.property_value(prop).get().unwrap()
                            })
                            .register_fn("get_obj", |obj: &mut gtk4::glib::Object, prop: &str| -> gtk4::glib::Object {
                                obj.property_value(prop).get().unwrap()
                            })

                            .register_fn("set_str", |obj: &mut gtk4::glib::Object, prop: &str, value: &str| {
                                obj.set_property(prop, value);
                            })
                            .register_fn("set_num", |obj: &mut gtk4::glib::Object, prop: &str, value: i64| {
                                obj.set_property(prop, value);
                            })
                            .register_fn("set_obj", |obj: &mut gtk4::glib::Object, prop: &str, value: gtk4::glib::Object| {
                                obj.set_property(prop, value);
                            })

                            .eval_ast_with_scope::<()>(&mut scope, &compiled);

                        // To prevent application closing
                        if let Err(err) = result {
                            println!("Failed to execute rhai script: {}", err);
                        }
    
                        None
                    }) {
                        break;
                    }
                }
            }
        }

        Ok(Self {
            builder
        })
    }

    pub fn object<T: gtk4::glib::IsA<gtk4::glib::Object>>(&self, name: &str) -> Option<T> {
        self.builder.object(name)
    }
}
