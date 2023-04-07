use std::sync::atomic::{AtomicUsize, Ordering};

use cursive::{menu, views::Dialog, With, event::Key};

// TODO: add key event handler: "M-F": highlight File, "M-H": highlight Help, "M-Q": highlight Quit

fn main() {
    let mut csrv = cursive::default();

    // counter to name a new file
    let counter = AtomicUsize::new(1);

    // the menubar is a list of (label, menu tree) pairs
    csrv.menubar()
        .add_subtree(
            // add a new "File" tree
            "File",
            menu::Tree::new()
                .leaf("New", move |s| {
                    // trees are made of leaves, which are directly
                    // actionable
                    // here we use the counter to add an entry in the list of "Recent" item
                    let i = counter.fetch_add(1, Ordering::Relaxed);
                    let filename = format!("New {i}");
                    s.menubar()
                        .find_subtree("File")
                        .unwrap()
                        .find_subtree("Recent")
                        .unwrap()
                        .insert_leaf(0, filename, |_| ());
                    s.add_layer(Dialog::info("New file"));
                })
                // ... and of sub-trees which open up when selected
                .subtree(
                    "Recent",
                    // '.with()' method is to help when running loops withing builder patterns
                    menu::Tree::new().with(|tree| {
                        for i in 1..10 {
                            tree.add_item(menu::Item::leaf(format!("Item {i}"), |_| ()).with(|s| {
                                if 0 == i % 5 {
                                    s.disable();
                                }
                            }))
                        }
                    }),
                )
                // delimiter are simple lines between items, and cannot be selected
                .delimiter()
                .with(|tree| {
                    for i in 1..10 {
                        tree.add_leaf(format!("Option {i}"), |_| ());
                    }
                }),
        )
        .add_subtree(
            "Help",
            menu::Tree::new()
                .subtree(
                    "Help",
                    menu::Tree::new()
                        .leaf("General", |s| s.add_layer(Dialog::info("Help message!")))
                        .leaf("Online", |s| {
                            let text = "Search it yourself!\n\
        Put some effort...";
                            s.add_layer(Dialog::info(text))
                        }),
                )
                .leaf("About", |s| {
                    s.add_layer(Dialog::info("Cusive Menu Example: v0.1"))
                }),
        )
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit());

    // csrv.set_autohide_menu(false);

    csrv.add_global_callback(Key::Esc, |s| s.select_menubar());
    csrv.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    csrv.run();
}
