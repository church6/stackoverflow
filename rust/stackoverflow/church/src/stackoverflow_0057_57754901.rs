// @filename   : stackoverflow_0057_57754901.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/57754901
// @title      : What is a "fat pointer"?

#[allow(dead_code)]
mod answer1 {
    /*
    The term "fat pointer" is used to refer to references and raw pointers to dynamically sized types (DSTs) – slices or trait objects. A fat pointer contains a pointer plus some information that makes the DST "complete" (e.g. the length).
    */

    mod code1 {
        // Slices ([T] and str)
        use std::mem::size_of;
        pub fn test() {
            // add your code here

            dbg!(size_of::<&u32>());
            dbg!(size_of::<&[u32; 2]>());
            dbg!(size_of::<&[u32]>());
            // This prints (with some cleanup):
            // size_of::<&u32>()      = 8
            // size_of::<&[u32; 2]>() = 8
            // size_of::<&[u32]>()    = 16

            assert_eq!(size_of::<&u32>(), 8);
            assert_eq!(size_of::<&[u32; 2]>(), 8);
            assert_eq!(size_of::<&[u32]>(), 16);
        }
    }
    mod code2 {
        // Trait objects (dyn Trait)
        use std::mem::size_of;
        trait Animal {
            fn speak(&self);
        }

        struct Cat;
        impl Animal for Cat {
            fn speak(&self) {
                println!("meow");
            }
        }

        pub fn test() {
            // add your code here

            dbg!(size_of::<&Cat>());
            dbg!(size_of::<&dyn Animal>());
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer3 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
pub fn test() {
    _enter!();
    answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
