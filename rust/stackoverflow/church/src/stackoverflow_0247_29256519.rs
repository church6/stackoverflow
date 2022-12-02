// @filename   : stackoverflow_0247_29256519.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/29256519
// @title      : I implemented a trait for another trait but cannot call methods from both traits

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // You need to implement the second trait for objects that implement the first trait:
        trait Sleep {
            fn sleep(&self);
        }

        struct Bed;
        impl Bed {
            fn jump(&self) {}
        }
        impl Sleep for Bed {
            fn sleep(&self) {}
        }

        struct Tent;
        impl Tent {
            fn hide(&self) {}
        }
        impl Sleep for Tent {
            fn sleep(&self) {}
        }

        struct Jim {
            bed: Bed,
        }
        struct Jane {
            tent: Tent,
        }

        fn example() {
            let jim = Jim { bed: Bed };
            jim.bed.sleep();
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

        pub trait Sleep: Sized {
            type Env: SleepEnv;

            fn sleep(&self, env: &Self::Env) {
                env.do_sleep(self);
            }

            fn get_name(&self) -> &'static str;
        }

        pub trait SleepEnv {
            fn do_sleep<T: Sleep>(&self, _: &T);
        }

        // Then, we implement two different sleep environments.

        struct Bed;
        struct Tent;

        impl SleepEnv for Bed {
            fn do_sleep<T: Sleep>(&self, person: &T) {
                println!("{} is sleeping in bed", person.get_name());
            }
        }

        impl SleepEnv for Tent {
            fn do_sleep<T: Sleep>(&self, person: &T) {
                println!("{} is sleeping in tent", person.get_name());
            }
        }

        // The last piece is the concrete implementations of them.

        struct Jim;
        struct Jane;

        impl Sleep for Jim {
            type Env = Bed;
            fn get_name(&self) -> &'static str {
                "Jim"
            }
        }

        impl Sleep for Jane {
            type Env = Tent;
            fn get_name(&self) -> &'static str {
                "Jane"
            }
        }

        fn example() {
            let bed = Bed;
            let tent = Tent;

            let jim = Jim;
            let jane = Jane;
            jim.sleep(&bed);
            jane.sleep(&tent);
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
