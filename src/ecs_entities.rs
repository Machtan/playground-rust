use complecs;
use ecs_components::*;
use ecs_processes::*;

entity_storage! {
    pub struct Entities {
        player: EPlayer,
    }
}

entity! {
    pub mod player {
        /// The avatar that the player controls in the game.
        pub struct EPlayer {
            name: CName, 
            age: CAge,
        }
        impl {
            PPrintInfo,
            PDoubleAge,
            PPrintWithLastName,
        }
    }
}
