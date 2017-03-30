#[macro_use]
extern crate complecs;

mod ecs_components;
mod ecs_processes;
mod ecs_entities;

use ecs_components::*;
use ecs_processes::*;
use ecs_entities::*;

#[derive(Debug, Default)]
pub struct Sim {
    components: Components,
    entities: Entities,
    processes: Processes,
}

impl Sim {
    pub fn new() -> Sim {
        Sim::default()
    }
    
    pub fn update(&mut self) {
        PPrintInfo::run(self);
        PDoubleAge::run(self);
        PPrintWithLastName::run(self, "Erroinen");
    }
}

contains_processes! {
    Sim.processes: Processes
}

contains_components! {
    Sim.components: Components
}

contains_entities! {
    Sim.entities: Entities
}

fn main() {
    println!("Hello world!");

    let mut sim = Sim::new();
    
    let player = EPlayer::new_data(String::from("Jakob"), 22);
    player.add_to(&mut sim);
    
    let another = EPlayer::new_data(String::from("test"), 9001);
    another.add_to(&mut sim);
    
    sim.update();
    sim.update();
}
