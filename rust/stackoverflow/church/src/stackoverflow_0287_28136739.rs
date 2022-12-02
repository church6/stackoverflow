// @filename   : stackoverflow_0287_28136739.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28136739
// @title      : Is it possible to control the size of an array using the type parameter of a generic?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Rust 1.51
        // Use const generics:

        /*
        #[derive(Debug)]
                struct Vec<T: Sized, const COUNT: usize> {
                    a: [T; COUNT],
                }
        */
        #[derive(Debug)]
        struct Point3D<T: Sized, const COUNT: usize> {
            a: [T; COUNT],
        }
        /*
        Previous versions
        RFC 2000 — const generics introduces support for this and progress is tracked in issue #44580.

        If you look at the design of Rust, you will notice that it started first by tackling the hardest problems (memory-safe, data-race free) but there are otherwise lots of areas where it is "incomplete" (compared to what could be achieved).

        In particular, generic structures and functions started out somewhat limited:

        lack of Higher Kinded Types (HKT)
        lack of non-type parameters => arrays are special-cased, and implementing a trait for an array is a known issue, the work-around being to implement it for a few different dimensions
        lack of variadic parameters => tuples are special-cased, and implementing a trait for all tuples is similarly difficult
        */

        fn example() {
            let data: Point3D<i32, 3> = {
                Point3D {
                    a: [1i32, 2i32, 3i32],
                }
            };
            println!("{:?}", data);
        }
        pub fn test() {
            // add your code here
            example();
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
        code1::test();
        //code2::test();
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
