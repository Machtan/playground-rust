extern crate froggy;

pub mod traits;
#[macro_use]
pub mod macros;

use traits::{EntityId, HasCompStore, HasProcStore, HasEntityStore, AddEntityToStore};
use froggy::{Storage};

macro_rules! process {
    (
        $( #[$meta:meta] )*
        pub mod $mod:ident {
            $proc_id:ident::run( 
                // Mutable components, always first.
                $( mut $mut_arg:ident[$mut_gensym:ident] : &mut $mut_comp:ident, )*
                
                // Immutable components.
                $( ref $arg:ident[$gensym:ident] : & $comp:ident, )*
                
                // External arguments (relevant here?)
                $( ext $ext_arg:ident : $ext_ty:ty, )*
            ) $body:block
        }
    ) => {
        $( #[$meta] )*
        pub mod $mod {
            use super::traits;
            use super::froggy;
            use std::fmt::Debug;
            $(
                use super::$mut_comp;
            )*
            $(
                use super::$comp;
            )*
            
            /// Indices to arguments of this process.
            pub type ArgRefs = ( $( froggy::StorageRc<<$comp as traits::CompId>::Type> ),* );
            
            // Arguments to this function
            // pub type Args = ( $( &mut $mut_comp, )* $( &$comp, )* );
            
            /// Identifies this process.
            pub struct $proc_id;
            
            impl traits::ProcId for $proc_id {
                type ArgRefs = self::ArgRefs;
            }
            
            unsafe impl<T> traits::HasProc<self::$proc_id> for T 
              where T: traits::HasProcStore<self::$proc_id>
                  $( + traits::HasCompStore<$mut_comp> )*
                  $( + traits::HasCompStore<$comp> )*
            {}
            
            impl $proc_id {
                pub fn run<S>(sim: &mut S $(, $ext_arg : $ext_ty )* )
                  where S: traits::HasProc<self::$proc_id> 
                         + traits::HasProcStore<self::$proc_id>
                      $( + traits::HasCompStore<$mut_comp> )*
                      $( + traits::HasCompStore<$comp> )*
                {
                    $(  
                        let mut $mut_arg = <S as traits::HasCompStore<$mut_comp>>::get_mut_components(sim).write();
                    )*
                    $(
                        let $arg = <S as traits::HasCompStore<$comp>>::get_components(sim).read();
                    )*
                    
                    for &( $( ref $mut_gensym, )* $( ref $gensym, )* )
                    in &<S as traits::HasProcStore<self::$proc_id>>::process_members(sim).read() {
                        $(
                            let $mut_arg = $mut_arg.get_mut($mut_gensym);
                        )*
                        $(
                            let $arg = $arg.get($gensym);
                        )*
                        $body
                    }
                }
            }
            
            // Add the debug clause to allow the concatenation of bounds.
            // Could as well be a useless blanket implemented trait.
            impl<T> traits::IntoProcArgs<self::$proc_id> for T
              where T: Debug 
                       $( + traits::HasComp<$mut_comp> )*
                       $( + traits::HasComp<$comp> )*
            {
                fn into_args(&self) -> self::ArgRefs {
                    (
                        $(<T as traits::HasComp<$mut_comp>>::get(self).clone() , )* 
                        $(<T as traits::HasComp<$comp>>::get(self).clone() , )*
                    )
                }
            }
        }
        
        // Bring the id into scope 
        pub use self::$mod::$proc_id;
    }
}

process! {
    pub mod print_info {
        PPrintInfo::run(ref name[n]: &CName, ref age[a]: &CAge,) { 
            println!("{} is {} year(s) old", name, age); 
        }
    }
}

/*pub type PrintInfoArgs = (StorageRc<String>, StorageRc<u32>);

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
}*/

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
            PPrintInfo
        }
    }
}

// ======= Processes =========

// ====== SIM data ====== 

#[derive(Debug, Default)]
struct Entities {
    players: Vec<player::ProcData>,
}

#[derive(Debug, Default)]
struct Processes {
    print_info: Storage<print_info::ArgRefs>,
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
}
