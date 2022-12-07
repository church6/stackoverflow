// @filename   : stackoverflow_0325_32872013.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/32872013
// @title      : Does Rust have a way to apply a function/method to each element in an array or vector?

#[allow(dead_code)]
mod answer1 {
    mod code1 {

        // Rust has Iterator::map, so you can:
        fn example1() {
            let some_vec = vec![1, 2, 3];
            // some_vec.iter().map(|x| x*2 /* do something here */);
            println!("{:?}", some_vec);
        }

        // However, Iterators are lazy so this won't do anything by itself. You can tack a .collect() onto the end to make a new vector with the new elements, if that's what you want:
        fn example2() {
            let some_vec = vec![1, 2, 3];
            let doubled: Vec<_> = some_vec.iter().map(|x| x * 2).collect();
            println!("{:?}", doubled);
        }

        // The standard way to perform side effects is to use a for loop:
        fn example3() {
            let some_vec = vec![1, 2, 3];
            for i in &some_vec {
                println!("{}", i);
            }
        }

        // If the side effect should modify the values in place, you can use an iterator of mutable references:
        fn example4() {
            let mut some_vec = vec![1, 2, 3];
            for i in &mut some_vec {
                *i *= 2;
            }
            println!("{:?}", some_vec); // [2, 4, 6]
        }

        // If you really want the functional style, you can use the .for_each() method:
        fn example5() {
            let mut some_vec = vec![1, 2, 3];
            some_vec.iter_mut().for_each(|i| *i *= 2);
            println!("{:?}", some_vec); // [2, 4, 6]
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
            example3();
            example4();
            example5();
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
        // Since Rust 1.21, the std::iter::Iterator trait defines a for_each() combinator which can be used to apply an operation to each element in the collection. It is eager (not lazy), so collect() is not needed:

        fn example() {
            let mut vec = vec![1, 2, 3];
            vec.iter_mut().for_each(|el| *el *= 2);
            println!("{:?}", vec);
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
    answer2::test();
    //answer3::test();
    _leave!();
}
