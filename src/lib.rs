mod utils;

use specs::*;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello!");
}

#[wasm_bindgen]
pub struct GameContainer {
    world: World,
}

#[wasm_bindgen]
impl GameContainer {
    pub fn create() -> GameContainer {
        GameContainer {
            world: World::new(),
        }
    }
    pub fn run_systems(&mut self) {
        let mut gls = GameLogSystem {};
        gls.run_now(&self.world);
    }
}

// Dummy System.
pub struct GameLogSystem {}
impl <'a>System<'a> for GameLogSystem {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        alert("its scot the woz");
    }
}