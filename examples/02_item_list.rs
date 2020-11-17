use rcui::*;

fn main() {
    rcui::exec(Proxy::wrap(
        |list, event| match event {
            Event::KeyStroke(key) => match *key {
                Some(pancurses::Input::Character('q')) => rcui::quit(),
                Some(pancurses::Input::Character('j')) => list.down(),
                Some(pancurses::Input::Character('k')) => list.up(),
                _ => {}
            }

            _ => {}
        },
        ItemList::new((0..100).map(|x| format!("item-{:02}", x)).collect()),
    ));
    println!("Quitting gracefully uwu");
}
