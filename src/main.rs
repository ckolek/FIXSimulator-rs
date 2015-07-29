
extern crate fix;

mod simulator;

use simulator::Simulator;
use fix::engine::Engine;

fn main() {
    let mut simulator : Simulator = Simulator::new("sim");
    
    let mut engine : Engine = Engine::new();
    engine.add_message_listener(&mut simulator);
}