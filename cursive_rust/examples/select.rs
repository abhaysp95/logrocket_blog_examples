use cursive::{views::{SelectView, Dialog, TextView, OnEventView}, align::HAlign, event::EventResult, view::{Scrollable, Resizable}};

fn main() {
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();

    let content = include_str!("assets/cities.txt");
    select.add_all_str(content.lines());

    // set the callback for when "Enter" is pressed
    select.set_on_submit(|s, city: &str| {
        s.pop_layer();
        let text = format!("{city} is a great city");
        s.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
    });

    // override 'j' and 'k' keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        }).on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    let mut csrv = cursive::default();
    csrv.add_layer(Dialog::around(select.scrollable().fixed_size((20, 10))).title("Where are you from?"));

    csrv.run();
}
