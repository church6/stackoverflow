// @filename   : stackoverflow_0326_52521201.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/52521201
// @title      : How do I synchronously return a value calculated in an asynchronous Future?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Standard library futures
        // Let's use this as our minimal, reproducible example:
        async fn calc() -> i32 {
            42
        }

        // Call executor::block_on:
        use futures::executor; // 0.3.25

        fn example() {
            let v = executor::block_on(calc());
            println!("code1={}", v);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        // use tokio; // 1.22.0
        async fn calc() -> i32 {
            42
        }

        #[tokio::main]
        async fn example() {
            let v = calc().await;
            println!("code2={}", v);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        use tokio::runtime::Runtime; // 1.22.0

        async fn calc() -> i32 {
            42
        }

        fn example() {
            let v = Runtime::new().unwrap().block_on(calc());
            println!("code3={}", v);
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
