extern crate froggy;

#[macro_export]
macro_rules! entity {
    (
        pub struct $entity:ident {
            id: pub struct $entity_id:ident;
            create: pub struct $create_name:ident;
            data: pub type $entity_data:ident;
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
        // Create the data used to add the item.
        #[derive(Debug)]
        pub struct $entity {
            $(
                pub $comp_name : <$comp_id as traits::CompId>::Type,
            )*
        }
        
        impl $entity {
            pub fn new( $( $comp_name : <$comp_id as traits::CompId>::Type ),* ) -> $entity {
                $entity {
                    $( $comp_name ),*
                }
            }
        }
        
        // Create an Id struct
        #[derive(Debug, Clone, Copy)]
        pub struct $entity_id;
        
        pub type $entity_data = ( $( froggy::StorageRc<<$proc_id as traits::ProcId>::ArgRefs> ),* ,);
        
        impl traits::EntityId for $entity_id {
            type Data = $entity_data;
        }
        
        impl<S> traits::AddEntityToStore<$entity_id, S> for $entity 
          where S: traits::HasEntityStore<$entity_id>
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
                let create_struct = $create_name {
                    $(
                        $comp_name
                    ),*
                };
                let entity = ( $(
                    sim.add_to_process( $proc_id , create_struct.clone() )
                ),* ,);
                sim.get_mut_entities($entity_id).push(entity);
            }
        }
        
        // Ensure that the entity can be added to processes
        #[derive(Debug, Clone)]
        pub struct $create_name {
            $(
                pub $comp_name : froggy::StorageRc<<$comp_id as traits::CompId>::Type>,
            )*
        }
        
        $(
            impl traits::HasComp<$comp_id> for $create_name {
                fn get(&self, _i: $comp_id) -> &froggy::StorageRc<<$comp_id as traits::CompId>::Type> {
                    &self.$comp_name
                }
            }
        )*
        
    }
}
