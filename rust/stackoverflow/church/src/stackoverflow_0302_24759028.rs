// @filename   : stackoverflow_0302_24759028.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/24759028
// @title      : How should you do pointer arithmetic in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {

        // Pointers have an offset method for pointer arithmetic.

        fn example1() {
            let items = [1usize, 2, 3, 4];

            let ptr = &items[1] as *const usize;

            println!("{}", unsafe { *ptr.offset(-1) }); //1
            println!("{}", unsafe { *ptr }); //2
            println!("{}", unsafe { *ptr.offset(1) }); //3
            println!("{}", unsafe { *ptr.offset(2) }); //4
        }

        fn example2() {
            let items = [1usize, 2, 3, 4];

            let ptr = items.as_ptr();

            unsafe {
                //println!("{}", *ptr.offset(-1)); //invalid
                println!("{}", *ptr); //1
                println!("{}", *ptr.offset(1)); //2
                println!("{}", *ptr.offset(2)); //3
                println!("{}", *ptr.offset(3)); //4
            }
        }

        fn example3() {
            let items = [1usize, 2, 3, 4];

            let ptr = &items[1] as *const usize;
            unsafe {
                println!("{}", *ptr.add(0));
                println!("{}", *ptr);
                println!("{}", *ptr.add(1));
                println!("{}", *ptr.add(2));
            }
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
            example3();
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
