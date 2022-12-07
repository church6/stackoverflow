// @filename   : stackoverflow_0310_29669287.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/29669287
// @title      : How can I zip more than two iterators?

#[allow(dead_code)]
mod answer1 {
    mod code1 {

        use itertools::izip;

        fn example() {
            let a = [1, 2, 3];
            let b = [4, 5, 6];
            let c = [7, 8, 9];

            // izip!() accepts iterators and/or values with IntoIterator.
            for (x, y, z) in izip!(&a, &b, &c) {
                println!("{x},{y},{z}");
            }
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

        macro_rules! zip {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        $x.iter().zip(
            zip!($($y), +))
    )
}

        fn example() {
            let x = vec![1, 2, 3];
            let y = vec![4, 5, 6];
            let z = vec![7, 8, 9];

            let zipped = zip!(x, y, z);
            println!("{:?}", zipped);
            for (a, (b, c)) in zipped {
                println!("{} {} {}", a, b, c);
            }
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
