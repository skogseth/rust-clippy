#![allow(dead_code)]
#![warn(clippy::non_hierarchical_crate_paths)]

fn main() {}

pub mod hello {
    // Valid
    use world::function1;
    use self::world::function2;

    // Invalid
    use crate::other::function3;
    use super::other::function4;
    
    pub mod world {
        pub fn function1() {}
        pub fn function2() {}
    }
}

pub mod other {
    pub fn function3() {}
    pub fn function4() {}
}
