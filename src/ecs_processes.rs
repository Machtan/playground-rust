use complecs;
use ecs_components::*;

process_storage! {
    pub struct Processes {
        print_info: PPrintInfo,
        double_age: PDoubleAge,
        pwln: PPrintWithLastName,
    }
}

process! {
    pub mod print_info {
        /// Prints info about an entity.
        pub fn PPrintInfo::run(ref name[n]: &CName, ref age[a]: &CAge,) { 
            println!("{} is {} year(s) old", name, age); 
        }
    }
}

process! {
    pub mod double_age {
        /// Doubles the age of an entity.
        pub fn PDoubleAge::run(mut age[a]: &mut CAge,) {
            *age *= 2;
        }
    }
}

process! {
    pub mod print_with_last_name {
        /// Prints the name of the entity with an added last name.
        pub fn PPrintWithLastName::run(ref name[n]: &CName, ext last_name: &str,) {
            println!("Name: {} {}", name, last_name);
        }
    }
}

