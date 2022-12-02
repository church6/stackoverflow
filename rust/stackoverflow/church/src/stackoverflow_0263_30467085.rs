// @filename   : stackoverflow_0263_30467085.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30467085
// @title      : How to iterate over and filter an array?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn hooray() {
            let array = [1, 4, 3, 2, 2];
            let array_iter = array.into_iter();
            // array_iter.filter(|&&x| x == 2);
            let iter = array_iter.filter(|&x| x == 2);
            println!("{:?}", array);
            assert_eq!((0, Some(5)), iter.size_hint());
        }

        pub fn test() {
            // add your code here
            hooray();
        }
    }
    mod code2 {
        fn hooray() {
            let array = [1, 4, 3, 2, 2];
            // deprecated 1.51
            //let array_iter = std::array::IntoIter::new(array);
            let array_iter = array.into_iter();
            let iter = array_iter.filter(|&x| x == 2);
            println!("{:?}", array);
            assert_eq!((0, Some(5)), iter.size_hint());
        }

        pub fn test() {
            // add your code here
            hooray();
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
