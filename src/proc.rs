extern crate froggy;

use froggy::{Storage, StorageRc};

pub trait CompId {
    type CompType: Debug;
}

pub trait Comp<I: CompId> {
    fn get(&self, i: I) -> &StorageRc<I::CompType>;
}

#[derive(Debug, Clone)]
pub struct PrintInfoArgs (StorageRc<String>, StorageRc<u32>);

impl<T> From<T> for PrintInfoArgs where T: Comp<NameId> + Comp<AgeId> {
    fn from(value: T) -> PrintInfoArgs {
        PrintInfoArgs(value.get(NameId).clone(), value.get(AgeId).clone())
    }
}

pub trait ProcId {
    type Args: Clone;
    type ExtraArgs;
}

pub struct PrintInfoProc;
impl ProcId for PrintInfoProc {
    type Args = PrintInfoArgs;
    type ExtraArgs = ();
}

pub trait Proc<P: ProcId> {
    
    fn add_to_process<E>(&mut self, _: P, e: E) -> <P as ProcId>::Args where E: Into<<P as ProcId>::Args>;
    fn process(&mut self, _: P, args: <P as ProcId>::ExtraArgs);
}

type PersonData = (StorageRc<PrintInfoArgs>,);

#[derive(Debug, Default)]
struct Components {
    names: Storage<String>,
    ages: Storage<u32>,
}

#[derive(Debug, Default)]
struct Entities {
    persons: Storage<PersonData>,
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
        self.process(PrintInfoProc, ());
    }
}

impl Proc<PrintInfoProc> for Sim {
    fn add_to_process<E>(&mut self, _: PrintInfoProc, e: E) -> PrintInfoArgs where E: Into<<PrintInfoProc as ProcId>::Args> {
        let args = e.into();
        self.p_print_info.write().insert(args.clone());
        args
    }
    
    fn process(&mut self, _: PrintInfoProc, _: ()) {
        let names = self.names.read();
        let ages = self.ages.read();
        for &PrintInfoArgs(ref name, ref age) in &self.p_print_info.read() {
            let name = names.get(name);
            let age = ages.get(age);
            println!("{} is {} year(s) old", name, age);
        }
    }
}

#[derive(Debug, Clone)]
struct PersonCreateArgs {
    name: StorageRc<String>,
    age: StorageRc<u32>,
}

pub struct NameId;
impl CompId for NameId {
    type CompType = String;
}

pub struct AgeId;
impl CompId for AgeId {
    type CompType = u32;
}

impl Comp<NameId> for PersonCreateArgs {
    fn get(&self, _i: NameId) -> &StorageRc<String> { &self.name }
}

impl Comp<AgeId> for PersonCreateArgs {
    fn get(&self, _i: AgeId) -> &StorageRc<u32> { &self.age }
}


fn main() {
    println!("Hello world!");
    /*let persson = Person { name: String::from("Markus"), age: 37 };
    print_age_name(&persson);*/
    let mut sim = Sim::new();
    let name = sim.names.write().insert(String::from("Markus"));
    let age = sim.ages.write().insert(37);
    let insert = PersonCreateArgs { name, age };
    let pi_data = sim.add_to_process(PrintInfoProc, insert.clone());
    sim.update();
}
