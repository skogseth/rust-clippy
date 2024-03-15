#![allow(dead_code)]
#![warn(clippy::non_hierarchical_crate_paths)]

fn main() {}

pub mod hello {
    pub mod inner_module {
        pub fn function1() {}
        pub fn function2() {}
        pub fn function3() {}
    }

    // Valid
    use self::inner_module::function2;
    use inner_module::function1;

    // Invalid (but valid object)
    use crate::hello::inner_module::function3;

    // Invalid
    use super::world::function5;
    use crate::world::function4;

    fn test() {
        // Valid
        inner_module::function1();
        self::inner_module::function2();

        // Invalid (but valid object)
        crate::hello::inner_module::function3();

        // Invalid
        crate::world::function4();
        super::world::function5();
    }
}

pub mod world {
    pub fn function4() {}
    pub fn function5() {}
}
