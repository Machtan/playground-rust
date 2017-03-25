
struct Test {
    name: String,
    age: u32,
}
impl Test {
    fn get_name(&mut self) -> *mut String { &mut self.name }
    
    // Oops
    fn get_pet_name(&mut self) -> *mut String { &mut self.name }
    
    fn get_age(&self) -> *const u32 { &self.age }
}

fn main() {
    println!("Yo world!");
    
    let mut t = Test {
        name: String::from("Testeroinen"),
        age: 28,
    };
    
    //let name = &mut t.name;
    //let age = &t.age;
    let name = unsafe { &mut *t.get_name() };
    let pet_name = unsafe { &mut *t.get_pet_name() };
    let age = unsafe { &*t.get_age() };
    
    *name = String::from("Erroinen");
    
    println!("Name: {}", name);
    println!("Age:  {}", age);
    println!("Pet name:  {}", pet_name);
    
}