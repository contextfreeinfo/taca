wit_bindgen::generate!({
    path: "../../../taca.wit",
    world: "taca",
});

use exports::taca::core::app::{EventKind, Guest};
use taca::core::console;
use taca::core::key;

struct TacaApp;
export!(TacaApp);

impl Guest for TacaApp {
    fn update(event_kind: EventKind) {
        match event_kind {
            EventKind::Frame => {
                console::print("Frame event");
            }
            EventKind::Key => {
                let event = key::get_event();
                let status = if event.pressed { "pressed" } else { "released" };
                let msg = format!("Key {:?} {}", event.code, status);
                console::print(&msg);
            }
            EventKind::TasksDone => {
                console::print("Tasks done event");
            }
        }
    }
}
