extern crate froggy;

pub mod traits;
#[macro_use]
pub mod macros;

use traits::{ProcId, CompId, EntityId, HasComp, HasProc, HasCompStore, 
    HasProcStore, HasEntityStore, AddEntityToStore, IntoProcArgs};
use froggy::{Storage, StorageRc}; 

macro_rules! contains_components {
    (
        $type:ty => $member:ident: $comp_type:ty
    ) => {
        impl<C> HasCompStore<C> for $type where C: CompId, $comp_type: HasCompStore<C> {
            fn get_mut_components(&mut self) -> &mut froggy::Storage<<C as CompId>::Type> {
                self.$member.get_mut_components()
            }

            fn get_components(&self) -> &froggy::Storage<<C as CompId>::Type> {
                self.$member.get_components()
            }
        }
    }
}

// ====== Component definitions ======
component! { CName: String }
component! { CAge: u32 }

component_storage! {
    /// Stores all the components!
    pub struct Components {
        names: CName,
        ages: CAge,
    }
}

contains_components! {
    Sim => components: Components
}

// ============= Macros ================

entity! {
    /// The avatar that the player controls in the game.
    pub mod player {
        components: {
            name: CName, 
            age: CAge,
        }
        processes: {
            PrintInfoProc
        }
    }
}

// ======= Processes =========

pub type PrintInfoArgs = (StorageRc<String>, StorageRc<u32>);

impl<T> IntoProcArgs<PrintInfoProc> for T where T: HasComp<CName> + HasComp<CAge> {
    fn into_args(&self) -> PrintInfoArgs {
        (<T as HasComp<CName>>::get(self).clone(), <T as HasComp<CAge>>::get(self).clone())
    }
}

unsafe impl<T> HasProc<PrintInfoProc> for T 
  where T: HasProcStore<PrintInfoProc>
         + HasCompStore<CName>
         + HasCompStore<CAge> 
{}

pub struct PrintInfoProc;
impl ProcId for PrintInfoProc {
    type ArgRefs = PrintInfoArgs;
    type ExtraArgs = ();
}

// ====== SIM data ====== 

#[derive(Debug, Default)]
struct Entities {
    players: Vec<player::ProcData>,
}

#[derive(Debug, Default)]
struct Processes {
    print_info: Storage<PrintInfoArgs>,
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
        {
            let names = <Sim as HasCompStore<CName>>::get_components(self).read();
            let ages = <Sim as HasCompStore<CAge>>::get_components(self).read();
            for &(ref name, ref age) in &self.process_members().read() {
                let name = names.get(name);
                let age = ages.get(age);
                println!("{} is {} year(s) old", name, age);
            }
        }
    }
}


impl HasProcStore<PrintInfoProc> for Sim {
    fn process_members_mut(&mut self) -> &mut Storage<PrintInfoArgs> {
        &mut self.processes.print_info
    }
    
    fn process_members(&self) -> &Storage<PrintInfoArgs> {
        &self.processes.print_info
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
    println!("print_info: {:?}", sim.processes.print_info);
    println!("players:    {:?}", sim.entities.players);
    
    sim.update();
}
