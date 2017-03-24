extern crate froggy;

/// Declares a new entity, with its members contained in the module with the given name.
///
/// # Generation example
/// ```
/// pub mod player {
///     // How the data should be stored
///     pub type ProcData = (StorageRc<C> for C in components);
///
///     pub struct Id; // Identifies the struct
///     impl EntityId for Id { 
///         Data = self::ProcData
///     }
///
///     pub struct Data { ... }; // Used to add the entity to a simulation
///     impl<S> AddEntityToStore<self::Id> for self::Data
///       where S: HasEntityStore<self::Id> 
///             (+ HasComp<C> for C in components)
///             (+ HasProc<P> for P in processes)
///     {
///         fn add_to(self, sim: &mut S) { ... }
///     }
///
///     pub struct CompRefs { ... } // Used internally
/// }
///
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
        
        /// The data that should be stored about this entity to keep it alive.
        pub type ProcData = ( $( froggy::StorageRc<<$proc_id as traits::ProcId>::ArgRefs> ),* ,);
        
        // Create an Id struct
        /// The identifier for an entity.
        #[derive(Debug, Clone, Copy)]
        pub struct Id;
        
        impl traits::EntityId for self::Id {
            type Data = self::ProcData;
        }
        
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
                    let $comp_name = <S as traits::HasCompStore<$comp_id>>::get_mut_components(sim).write().insert(self.$comp_name);
                )*
                let components = CompRefs {
                    $(
                        $comp_name
                    ),*
                };
                let entity = ( $(
                    <S as traits::HasProc<$proc_id>>::add_to_process(sim, components.clone() )
                ),* ,);
                <S as traits::HasEntityStore<self::Id>>::get_mut_entities(sim).push(entity);
            }
        }
        
        // Ensure that the entity can be added to processes
        /// A struct holding references to the components of this entity inside
        /// a store. 
        /// 
        /// Used internally to access the right components to get the arguments
        /// for a process when adding the entity to it.
        #[derive(Debug, Clone)]
        pub struct CompRefs {
            $(
                /// A component.
                pub $comp_name : froggy::StorageRc<<$comp_id as traits::CompId>::Type>,
            )*
        }
        
        $(
            impl traits::HasComp<$comp_id> for self::CompRefs {
                fn get(&self) -> &froggy::StorageRc<<$comp_id as traits::CompId>::Type> {
                    &self.$comp_name
                }
            }
        )*
    }
    }
}
