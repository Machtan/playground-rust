use complecs;

component_storage! {
    /// Stores all the components!
    pub struct Components {
        names: CName,
        ages: CAge,
    }
}

component! { 
    /// The name of an entity.
    pub CName: String
}
component! {
    /// The age of an entity.
    pub CAge: u32
}
