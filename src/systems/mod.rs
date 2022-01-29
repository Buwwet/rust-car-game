use shred::{DispatcherBuilder, System, Dispatcher};
use wasm_bindgen::prelude::wasm_bindgen;

// Import our systems and create a
// Dispatcher our of it.

pub fn system_dispatcher() -> Dispatcher<'static, 'static> {
    // A Dispatcher can setup Systems for a World and
    // also run them in paralel.
    let mut dispatcher = DispatcherBuilder::new()
        .with(GameLogSystem {}, "GameLogSystem", &[])
        .build();

    dispatcher
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// Dummy System.
pub struct GameLogSystem {}
impl <'a>System<'a> for GameLogSystem {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        alert("its scot the woz");
    }
}

