
use std::fmt::Debug;
use froggy::{Storage, StorageRc};

/// Identifies and describes a component; a data member of an entity.
pub trait CompId {
    /// The data type of this component (not wrapped in anything).
    /// 
    /// Example: `u32`.
    type Type: Debug;
}

/// Signifies that the object stores an index to a component of the identified type.
pub trait HasComp<C: CompId> {
    #[inline]
    fn get(&self, i: C) -> &StorageRc<C::Type>;
}

// TODO: Processes are 'weaker' than entities now, see if they can be improved
/// Identifies and describes a process.
pub trait ProcId {
    /// What arguments of the process should be stored. (a data type containing `StorageRc`es )
    ///
    /// Think of it as the arguments to the function, but wrapped in storage indices
    /// so for `print_info(age: &u32, name: &String)` this would be:
    /// 
    /// Example: `(StorageRc<u32>, StorageRc<String>)`.
    type Args: Clone;
    /// What additional arguments should be passed through to it.
    ///
    /// Example: `(&mut Renderer, &Input)`.
    type ExtraArgs;
}

/// Signifies that the object contains a storage for arguments to the
/// identified process. (A list of entity components).
pub trait HasProcStore<P: ProcId> {
    /// Returns a mutable reference to the store of arguments to the process.
    #[inline]
    fn process_members_mut(&mut self, _: P) -> &mut Storage<P::Args>;
    
    /// Returns an immutable reference to the store of arguments to the process.
    #[inline]
    fn process_members(&self, _: P) -> & Storage<P::Args>;
    
    /// Runs the given function for each entity in this process, passing 
    /// references to the components of the entity as arguments.
    #[inline]
    fn process_each<F>(&self, p: P, mut f: F) where F: FnMut(&P::Args) {
        for arg in &self.process_members(p).read() {
            f(arg);
        }
    }
}

/// under construction.
pub trait HasProc<P: ProcId> : HasProcStore<P> {
    /// Adds an entity to this process, by giving storage indices to its components.
    #[inline]
    fn add_to_process<E>(&mut self, p: P, e: E) -> StorageRc<P::Args> 
      where E: Into<P::Args> 
    {
        self.process_members_mut(p).write().insert(e.into())
    }
    
    //fn process<F, A>(&mut self, _: P, extra: P::ExtraArgs, mut f: F) where F: FnMut(A, P::ExtraArgs);
}

/// Signifies that the object contains a storage for components of the identified type.
pub trait HasCompStore<C: CompId> {
    /// Returns a mutable reference to the component store.
    #[inline]
    fn get_mut_components(&mut self, _: C) -> &mut Storage<C::Type>;
    
    /// Returns an immutable reference to the component store.
    #[inline]
    fn get_components(&self, _: C) -> &Storage<C::Type>;
}

/// Identifies and describes an entity; a named collection of components and behaviors.
pub trait EntityId {
    /// The data type that is stored for this entity in the simulation/system/world.
    /// 
    /// This should be a type that contains references to all the process arguments
    /// that 'belong' to this entity.
    ///
    /// Example: `(StorageRc<<RenderProc as ProcId>::Args>, StorageRc<<MoveProc as ProcId>::Args>)`.
    type Data: Debug;
}

/// Signifies that the object contains a storage for entities of the identified type.
pub trait HasEntityStore<E: EntityId> {
    /// Returns a mutable reference to the entity store.
    #[inline]
    fn get_mut_entities(&mut self, _: E) -> &mut Vec<E::Data>;
}

/// Signifies that the entity can be added to a simulation that fulfils a
/// set of requirements.
///
/// `<S>` should be constrained to include the required components and processes.
///
/// # Example
/// ```no_run
/// impl<S> AddEntityToStore<EPlayer, S> for Player 
///   where S: HasEntityStore<EPlayer>
///          + HasCompStore<CName>
///          + HasCompStore<CAge>
///          + HasProc<PPrintNameAge>
/// {
///     fn add_to(self, sim: &mut S) { ... }
/// ```
pub trait AddEntityToStore<E: EntityId, S: HasEntityStore<E>> {
    fn add_to(self, sim: &mut S);
}