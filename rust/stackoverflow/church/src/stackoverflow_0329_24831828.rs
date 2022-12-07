// @filename   : stackoverflow_0329_24831828.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/24831828
// @title      : How do I pass an array to a function in Rust and change its content?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // accept &mut [i32] as the function argument, not &[i32]
        // pass &mut arr to the function, not &arr:

        fn change_value(arr: &mut [i32]) {
            arr[1] = 10;
        }

        fn example() {
            let mut arr: [i32; 4] = [1, 2, 3, 4];
            change_value(&mut arr);
            println!("this is {}", arr[1]);
            assert_eq!(arr[1], 10);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        // If you wanted to accept an array instead of a slice, you could also do that:
        fn change_value(arr: &mut [i32; 4]) {
            arr[1] = 10;
        }

        fn example() {
            let mut arr: [i32; 4] = [1, 2, 3, 4];
            change_value(&mut arr);
            println!("this is {}", arr[1]);
            assert_eq!(arr[1], 10);
        }
        pub fn test() {
            // add your code here
            example();
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
