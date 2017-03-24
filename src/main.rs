extern crate froggy;

mod traits;
#[macro_use]
mod macros;

use traits::{ProcId, CompId, EntityId, HasComp, HasProc, HasCompStore, HasProcStore, HasEntityStore, AddEntityToStore};
use froggy::{Storage, StorageRc}; 

// ============= Macros ================

// TODO: Should this make a micro-module for namespacing purposes?
// It's super annoying to have so many gensym arguments.

entity! {
    pub struct Player {
        id: pub struct PlayerId;
        create: pub struct CreatePlayer;
        data: pub type PlayerData;
        components: {
            name: NameId, 
            age: AgeId,
        }
        processes: {
            PrintInfoProc
        }
    }
}

// ======= Processes =========

#[derive(Debug, Clone)]
pub struct PrintInfoArgs (StorageRc<String>, StorageRc<u32>);

impl<T> From<T> for PrintInfoArgs where T: HasComp<NameId> + HasComp<AgeId> {
    fn from(value: T) -> PrintInfoArgs {
        PrintInfoArgs(value.get(NameId).clone(), value.get(AgeId).clone())
    }
}

pub struct PrintInfoProc;
impl ProcId for PrintInfoProc {
    type Args = PrintInfoArgs;
    type ExtraArgs = ();
}

// ====== SIM data ====== 
#[derive(Debug, Default)]
struct Components {
    names: Storage<String>,
    ages: Storage<u32>,
}

#[derive(Debug, Default)]
struct Entities {
    players: Vec<PlayerData>,
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
        /*{
            let names = self.components.names.read();
            let ages = self.components.ages.read();
            self.for_each(PrintInfoProc, |&PrintInfoArgs(ref name, ref age)| {
                let name = names.get(name);
                let age = ages.get(age);
                println!("{} is {} year(s) old", name, age);
            });
        }*/
    }
}

impl HasProcStore<PrintInfoProc> for Sim {
    fn process_members_mut(&mut self, _: PrintInfoProc) -> &mut Storage<PrintInfoArgs> {
        &mut self.processes.print_info
    }
    
    fn process_members(&self, _: PrintInfoProc) -> &Storage<PrintInfoArgs> {
        &self.processes.print_info
    }
}

impl HasEntityStore<PlayerId> for Sim {
    fn get_mut_entities(&mut self, _: PlayerId) -> &mut Vec<<PlayerId as EntityId>::Data> {
        &mut self.entities.players
    }
}

impl HasCompStore<NameId> for Sim {
    fn get_mut_components(&mut self, _: NameId) -> &mut Storage<<NameId as CompId>::Type> {
        &mut self.components.names
    }
    
    fn get_components(&self, _: NameId) -> &Storage<<NameId as CompId>::Type> {
        &self.components.names
    }
}

impl HasCompStore<AgeId> for Sim {
    fn get_mut_components(&mut self, _: AgeId) -> &mut Storage<<AgeId as CompId>::Type> {
        &mut self.components.ages
    }
    
    fn get_components(&self, _: AgeId) -> &Storage<<AgeId as CompId>::Type> {
        &self.components.ages
    }
}

// Blanket, woo!
impl<T> HasProc<PrintInfoProc> for T 
  where T: HasProcStore<PrintInfoProc> 
         + HasCompStore<NameId>
         + HasCompStore<AgeId>
{
    /*fn process<F, (&String, &u32)>(&mut self, _: PrintInfoProc, extra: (), mut f: F) where A: , F: FnMut(A, ()) {
        let names = self.get_components(NameId).read();
        let ages = self.get_components(AgeId).read();
        self.process_each(PrintInfoProc, |&PrintInfoArgs(ref name, ref age)| {
            let name = names.get(name);
            let age = ages.get(age);
            f((name, age), ());
        });
    }*/
}

// ====== Component definitions ======
pub struct NameId;
impl CompId for NameId {
    type Type = String;
}

pub struct AgeId;
impl CompId for AgeId {
    type Type = u32;
}


fn main() {
    println!("Hello world!");

    let mut sim = Sim::new();
    
    let player = Player::new(String::from("Jakob"), 22);
    player.add_to(&mut sim);
    
    let another = Player::new(String::from("test"), 9001);
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
