<h1 align="center">🦀 gtk-ui-builder</h1>

A Rust library to parse Blueprint files and convert them into GTK UI files

Inspired by the [Blueprint](https://gitlab.gnome.org/jwestman/blueprint-compiler) project

## Blueprint file

```
using Gtk 4.0;
using Adw 1;

Adw.ApplicationWindow window {
    default-width: 600;
    default-height: 500;

    content: Gtk.Box {
        orientation: vertical;

        Adw.HeaderBar {
            title-widget: Adw.WindowTitle {
                title: "Example app";
            };
        }

        Adw.PreferencesPage {
            Adw.PreferencesGroup {
                vexpand: true;
                valign: center;

                Gtk.Button {
                    label: "Hello, World!";
                }
            }
        }
    };
}
```

## Translation into XML format

```rs
use gtk_ui_builder::prelude::*;

fn main() {
    // Read main.blp file
    let pattern = std::fs::read_to_string("assets/ui/main.blp")
        .expect("Failed to read pattern");

    // Parse AST
    let tree = Parser::parse(pattern)
        .expect("Failed to parse blueprint");

    // Output prettified AST
    println!("{}", tree.root.dbg());

    // Get XML representation of this AST
    let ui = tree.get_xml();

    // Write this representation to the file
    // now you can import it as any GTK UI file
    std::fs::write("assets/ui/main.ui", &ui);
}
```

## Importing blueprint in GTK app

```rs
// We're using gtk-builder feature here
use gtk_ui_builder::prelude::*;

fn main() {
    gtk4::init().expect("GTK initialization failed");
    libadwaita::init();

    // Create app
    let application = gtk::Application::new(
        Some("com.github.krypt0nn.gtk-ui-builder"),
        Default::default()
    );

    // Init app window and show it
    application.connect_activate(|app| {
        // You also can parse blueprint with Parser::parse
        // and then use it in gtk4::Builder
        let builder = Builder::new(include_str!("../assets/ui/main.blp"))
            .expect("Failed to parse blueprint");

        let window = builder.object::<adw::ApplicationWindow>("window").unwrap();

        window.set_application(Some(app));
        window.show();
    });

    // Run app
    application.run();
}
```

Author: [Nikita Podvirnyy](https://vk.com/technomindlp)

Licensed under [GNU GPL 3.0](LICENSE)
