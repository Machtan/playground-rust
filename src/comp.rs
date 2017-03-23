
trait CompId {
    type CompType;
}

trait Comp<I: CompId> {
    fn get(&self, i: I) -> &I::CompType;
}

struct Person {
    name: String,
    age: u32,
}

struct NameId;
impl CompId for NameId {
    type CompType = String;
}

struct AgeId;
impl CompId for AgeId {
    type CompType = u32;
}

impl Comp<NameId> for Person {
    fn get(&self, _i: NameId) -> &String { &self.name }
}

impl Comp<AgeId> for Person {
    fn get(&self, _i: AgeId) -> &u32 { &self.age }
}

fn print_age_name<T>(t: &T) where T: Comp<NameId> + Comp<AgeId> {
    println!("[name: {}, age: {}]", t.get(NameId), t.get(AgeId));
}


fn main() {
    println!("Hello world!");
    let persson = Person { name: String::from("Markus"), age: 37 };
    print_age_name(&persson);
}
