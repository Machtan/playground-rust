
use std::fmt::Debug;
use froggy::{Storage, StorageRc};

/// Identifies and describes a component; a data member of an entity.
pub trait CompId {
    /// The data type of this component (not wrapped in anything).
    /// 
    /// Example: `u32`.
    type Type: Debug;
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

// TODO: Processes are 'weaker' than entities now, see if they can be improved
/// Identifies and describes a process.
pub trait ProcId {
    /// What arguments of the process should be stored. (a data type containing `StorageRc`es )
    ///
    /// Think of it as the arguments to the function, but wrapped in storage indices
    /// so for `print_info(age: &u32, name: &String)` this would be:
    /// 
    /// Example: `(StorageRc<u32>, StorageRc<String>)`.
    type ArgRefs: Clone;
    
    /// What additional arguments should be passed through to it.
    ///
    /// Example: `(&mut Renderer, &Input)`.
    type ExtraArgs;
}

pub trait IntoProcArgs<P: ProcId> {
    fn into_args(&self) -> <P as ProcId>::ArgRefs;
}

/// Signifies that the object stores an index to a component of the identified type.
pub trait HasComp<C: CompId> {
    #[inline]
    fn get(&self) -> &StorageRc<C::Type>;
}

/// Signifies that the object contains a storage for arguments to the
/// identified process. (A list of entity components).
pub trait HasProcStore<P: ProcId> {
    /// Returns a mutable reference to the store of arguments to the process.
    #[inline]
    fn process_members_mut(&mut self) -> &mut Storage<P::ArgRefs>;
    
    /// Returns an immutable reference to the store of arguments to the process.
    #[inline]
    fn process_members(&self) -> & Storage<P::ArgRefs>;
}

/// Signifies that the object has the required components to add entities 
/// to the identified process.
///
/// It is not truly `unsafe` to implement the trait, but can lead to logic errors
/// if the trait bounds are not set correctly.
///
/// `Self` (the implementor) should be constrained to require the components 
/// and processes needed by the identified process
/// 
/// # example
/// ```no_run
/// unsafe impl<T> HasProc<AgePrinter> for T 
///   where T: HasProcStore<AgePrinter>
///          + HasCompStore<AgeId> 
/// {}
/// ```
pub unsafe trait HasProc<P: ProcId> : HasProcStore<P> {
    
    /// Adds an entity to this process, by giving storage indices to its components.
    #[inline]
    fn add_to_process<E>(&mut self, e: E) -> StorageRc<P::ArgRefs> 
      where E: IntoProcArgs<P> 
    {
        self.process_members_mut().write().insert(e.into_args())
    }
}

/// Signifies that the object contains a storage for components of the identified type.
pub trait HasCompStore<C: CompId> {
    /// Returns a mutable reference to the component store.
    #[inline]
    fn get_mut_components(&mut self) -> &mut Storage<C::Type>;
    
    /// Returns an immutable reference to the component store.
    #[inline]
    fn get_components(&self) -> &Storage<C::Type>;
}

/// Signifies that the object contains a storage for entities of the identified type.
pub trait HasEntityStore<E: EntityId> {
    /// Returns a mutable reference to the entity store.
    #[inline]
    fn get_mut_entities(&mut self) -> &mut Vec<E::Data>;
}

/// Signifies that the entity can be added to a simulation that fulfils a
/// set of requirements.
///
/// It is not truly `unsafe` to implement the trait, but can lead to logic errors
/// if the trait bounds are not set correctly.
///
/// `<S>` should be constrained to include the required components and processes.
///
/// # Example
/// ```no_run
/// unsafe impl<S> AddEntityToStore<EPlayer, S> for Player 
///   where S: HasEntityStore<EPlayer>
///          + HasCompStore<CName>
///          + HasCompStore<CAge>
///          + HasProc<PPrintNameAge>
/// {
///     fn add_to(self, sim: &mut S) { ... }
/// ```
pub unsafe trait AddEntityToStore<E: EntityId, S: HasEntityStore<E>> {
    fn add_to(self, sim: &mut S);
}
