extern crate froggy;

pub mod traits;
#[macro_use]
pub mod macros;

use traits::{EntityId, HasCompStore, HasProcStore, HasEntityStore, AddEntityToStore};
use froggy::{Storage};

// ============ Components =============
component! { 
    /// The name of an entity.
    pub CName: String
}
component! {
    /// The age of an entity.
    pub CAge: u32
}

component_storage! {
    /// Stores all the components!
    pub struct Components {
        names: CName,
        ages: CAge,
    }
}

// ============= Processes ================

process! {
    pub mod print_info {
        /// Prints info about an entity.
        pub fn PPrintInfo::run(ref name[n]: &CName, ref age[a]: &CAge,) { 
            println!("{} is {} year(s) old", name, age); 
        }
    }
}

process! {
    pub mod double_age {
        /// Doubles the age of an entity.
        pub fn PDoubleAge::run(mut age[a]: &mut CAge,) {
            *age *= 2;
        }
    }
}

// ============= Entities ================

entity! {
    /// The avatar that the player controls in the game.
    pub mod player {
        components: {
            name: CName, 
            age: CAge,
        }
        processes: {
            PPrintInfo,
            PDoubleAge,
        }
    }
}

// ====== SIM data ====== 

#[derive(Debug, Default)]
struct Entities {
    players: Vec<player::ProcData>,
}

#[derive(Debug, Default)]
struct Processes {
    print_info: Storage<print_info::ArgRefs>,
    double_age: Storage<double_age::ArgRefs>,
}

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
    }
}


impl HasProcStore<PPrintInfo> for Sim {
    fn process_members_mut(&mut self) -> &mut Storage<print_info::ArgRefs> {
        &mut self.processes.print_info
    }
    
    fn process_members(&self) -> &Storage<print_info::ArgRefs> {
        &self.processes.print_info
    }
}

impl HasProcStore<PDoubleAge> for Sim {
    fn process_members_mut(&mut self) -> &mut Storage<double_age::ArgRefs> {
        &mut self.processes.double_age
    }
    
    fn process_members(&self) -> &Storage<double_age::ArgRefs> {
        &self.processes.double_age
    }
}

impl HasEntityStore<player::Id> for Sim {
    fn get_mut_entities(&mut self) -> &mut Vec<<player::Id as EntityId>::Data> {
        &mut self.entities.players
    }
}

fn main() {
    println!("Hello world!");

    let mut sim = Sim::new();
    
    let player = player::Data::new(String::from("Jakob"), 22);
    player.add_to(&mut sim);
    
    let another = player::Data::new(String::from("test"), 9001);
    another.add_to(&mut sim);
    
    //println!("\n==== BEFORE WRITE ====\n");
    //println!("print_info: {:?}", sim.processes.print_info);
    //println!("players:    {:?}", sim.entities.players);
    
    //sim.processes.print_info.write();
    
    //println!("\n==== AFTER WRITE ====\n");
    
    //println!("Sim: {:?}", sim);
    //println!("print_info: {:?}", sim.processes.print_info);
    //println!("players:    {:?}", sim.entities.players);
    
    sim.update();
    sim.update();
}
