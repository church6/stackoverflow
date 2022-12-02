// @filename   : stackoverflow_0161_16946888.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/16946888
// @title      : Is it possible to make a recursive closure in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example() {
            struct Fact<'s> {
                f: &'s dyn Fn(&Fact, u32) -> u32,
            }
            let fact = Fact {
                f: &|fact, x| if x == 0 { 1 } else { x * (fact.f)(fact, x - 1) },
            };

            println!("{}", (fact.f)(&fact, 5));
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        fn example() {
            fn fact(x: u32) -> u32 {
                if x == 0 {
                    1
                } else {
                    x * fact(x - 1)
                }
            }

            println!("{}", fact(5));
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        fn example() {
            struct FactEnv {
                base_case: u32,
            }
            fn fact(env: &FactEnv, x: u32) -> u32 {
                if x == 0 {
                    env.base_case
                } else {
                    x * fact(env, x - 1)
                }
            }

            let env = FactEnv { base_case: 1 };
            println!("{}", fact(&env, 5));
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
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