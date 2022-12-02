// @filename   : stackoverflow_0095_59081890.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/59081890
// @title      : Is it possible to write Quake's fast InvSqrt() function in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn inv_sqrt(x: f32) -> f32 {
            let i = x.to_bits();
            let i = 0x5f3759df - (i >> 1);
            let y = f32::from_bits(i);

            y * (1.5 - 0.5 * x * y * y)
        }

        fn print_both(v: f32) {
            println!("quake: {}", inv_sqrt(v));
            println!("real:  {}", 1.0 / v.sqrt());
            println!();
        }
        #[allow(clippy::approx_constant)]
        pub fn test() {
            // add your code here
            print_both(4.0);
            print_both(10.0);
            print_both(3.1415);
            print_both(std::f32::consts::PI);
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
