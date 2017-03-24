extern crate froggy;

#[macro_export]
macro_rules! entity {
    (
        $entity:ident: {
            components: {
                $(
                    $comp_name:ident : $comp_id:ident,
                )*
            }
            processes: {
                $( $proc_id:ident ),*
            }
        }
    ) => {
    /// Namespaced entity declaration [macro-generated].
    pub mod $entity {
        use super::traits;
        use super::froggy;
        $(
            use super::$comp_id;
        )*
        $(
            use super::$proc_id;
        )*
        
        // Create the data used to add the item.
        /// Entity creation data.
        #[derive(Debug)]
        pub struct Data {
            $(
                /// A component.
                pub $comp_name : <$comp_id as traits::CompId>::Type,
            )*
        }
        
        impl Data {
            /// Creates a new set of entity data.
            pub fn new( $( $comp_name : <$comp_id as traits::CompId>::Type ),* ) -> Data {
                Data {
                    $( $comp_name ),*
                }
            }
        }
        
        // Create an Id struct
        /// The identifier for an entity.
        #[derive(Debug, Clone, Copy)]
        pub struct Id;
        
        /// The data that should be stored about this entity to keep it alive.
        pub type ProcData = ( $( froggy::StorageRc<<$proc_id as traits::ProcId>::ArgRefs> ),* ,);
        
        impl traits::EntityId for self::Id {
            type Data = self::ProcData;
        }
        
        unsafe impl<S> traits::AddEntityToStore<self::Id, S> for self::Data 
          where S: traits::HasEntityStore<self::Id>
                $(
                    + traits::HasCompStore<$comp_id>
                )*
                $(
                    + traits::HasProc<$proc_id>
                )*
        {
            fn add_to(self, sim: &mut S) {
                $(
                    let $comp_name = sim.get_mut_components($comp_id).write().insert(self.$comp_name);
                )*
                let components = CompRefs {
                    $(
                        $comp_name
                    ),*
                };
                let entity = ( $(
                    sim.add_to_process( $proc_id , components.clone() )
                ),* ,);
                sim.get_mut_entities(self::Id).push(entity);
            }
        }
        
        // Ensure that the entity can be added to processes
        /// A struct holding references to the components of this entity inside
        /// a store. Used when adding the entity to processes.
        #[derive(Debug, Clone)]
        pub struct CompRefs {
            $(
                /// A component.
                pub $comp_name : froggy::StorageRc<<$comp_id as traits::CompId>::Type>,
            )*
        }
        
        $(
            impl traits::HasComp<$comp_id> for self::CompRefs {
                fn get(&self, _i: $comp_id) -> &froggy::StorageRc<<$comp_id as traits::CompId>::Type> {
                    &self.$comp_name
                }
            }
        )*
    }
    }
}
