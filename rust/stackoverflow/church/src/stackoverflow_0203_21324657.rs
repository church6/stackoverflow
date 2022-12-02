// @filename   : stackoverflow_0203_21324657.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/21324657
// @title      : Does Rust support Ruby-like string interpolation?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example1() {
            let a = "AAA";
            let b = format!("BBB {}", a);
            println!("{b}");
        }
        // output: BBB AAA

        fn example2() {
            let (person, species, name) = ("Charlie Brown", "dog", "Snoopy");

            // implicit named argument `person`
            print!("Hello {person}");

            // implicit named arguments `species` and `name`
            format!("The {species}'s name is {name}.");
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
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
