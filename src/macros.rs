extern crate froggy;

/// Declares a component with the given name and type.
#[macro_export]
macro_rules! component {
    ( $name:ident : $type:ty) => {
        /// A component.
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
        
        impl traits::CompId for $name {
            type Type = $type;
        }
    }
}

/// Declares a component with the members inside a created module.
/// The process takes a set of mutable and immutable components as arguments,
/// as declared with the `mut` and `ref` arguments.
/// 
/// The body of the run function is executed in a context, in which the
/// components have been loaded and converted to their associated types.
/// The run function can also have extra `ext` arguments declared, that
/// are just passed directly to the scope.
#[macro_export]
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

/// Declares a new entity, with its members contained in the module with the given name.
///
/// # Generation example
/// ```
/// /// <Your docstrings and/or attributes here>
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
        $( #[ $mod_meta:meta ] )*
        pub mod $entity:ident {
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
    $( #[ $mod_meta ] )*
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

/// Declares a storage type for the identified components.
#[macro_export]
macro_rules! component_storage {
    // No trailing comma
    (
        $( #[$storage_meta:meta] )*
        pub struct $storage:ident {
            $(
                $member:ident : $component:ty
            ),*
        }
    ) => {
        $( #[ $storage_meta ] )*
        #[derive(Debug, Default)]
        pub struct $storage {
            $(
                /// A component [macro-generated].
                pub $member : froggy::Storage<<$component as traits::CompId>::Type>
            ),*
        }
        
        $(
            impl HasCompStore<$component> for $storage {
                fn get_mut_components(&mut self) -> &mut froggy::Storage<<$component as traits::CompId>::Type> {
                    &mut self.$member
                }
    
                fn get_components(&self) -> &froggy::Storage<<$component as traits::CompId>::Type> {
                    &self.$member
                }
            }
        )*
    };
    // Trailing comma alias
    (
        $( #[$storage_meta:meta] )*
        pub struct $storage:ident {
            $(
                $member:ident : $component:ident,
            )*
        }
    ) => {
        component_storage! {
            $( #[$storage_meta] )*
            pub struct $storage {
                $(
                    $member : $component
                ),*
            }
        }
    }
}

/// Describes that all components stored by the member of the type is also
/// stored by the type.
#[macro_export]
macro_rules! contains_components {
    (
        $type:ty => $member:ident: $comp_type:ty
    ) => {
        impl<C> traits::HasCompStore<C> for $type where C: traits::CompId, $comp_type: HasCompStore<C> {
            fn get_mut_components(&mut self) -> &mut froggy::Storage<<C as traits::CompId>::Type> {
                self.$member.get_mut_components()
            }

            fn get_components(&self) -> &froggy::Storage<<C as traits::CompId>::Type> {
                self.$member.get_components()
            }
        }
    }
}
