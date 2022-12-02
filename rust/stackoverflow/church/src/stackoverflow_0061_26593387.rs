// @filename   : stackoverflow_0061_26593387.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/26593387
// @title      : How can I get the current time in milliseconds?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Since Rust 1.8, you do not need to use a crate. Instead, you can use SystemTime and UNIX_EPOCH:
        use std::time::{SystemTime, UNIX_EPOCH};
        // If you need exactly milliseconds, you can convert the Duration.
        pub fn test() {
            // add your code here
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            println!("{:?}", since_the_epoch);
        }
    }
    mod code2 {
        use std::time::{SystemTime, UNIX_EPOCH};
        pub fn test() {
            // add your code here

            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            //Rust 1.33
            let in_ms = since_the_epoch.as_millis();
            println!("ms={}", in_ms);

            //Rust 1.27
            let in_ms =
                since_the_epoch.as_secs() as u128 * 1000 + since_the_epoch.subsec_millis() as u128;
            println!("ms={}", in_ms);

            //Rust 1.8
            let in_ms = since_the_epoch.as_secs() * 1000
                + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
            println!("ms={}", in_ms);
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
        use std::time::Instant;
        pub fn test() {
            // add your code here
            let start = Instant::now();

            // do stuff

            let elapsed = start.elapsed();

            // Debug format
            println!("Debug: {:?}", elapsed);

            // Format as milliseconds rounded down
            // Since Rust 1.33:
            println!("Millis: {} ms", elapsed.as_millis());

            // Before Rust 1.33:
            // calling `subsec_millis()` is more concise than this calculation
            // println!("Millis: {} ms", (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64 );
            println!(
                "Millis: {} ms",
                (elapsed.as_secs() * 1_000) + elapsed.subsec_millis() as u64
            );
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
        use chrono::prelude::*;

        pub fn get_unix_timestamp_ms() -> i64 {
            let now = Utc::now();
            now.timestamp_millis()
        }

        pub fn get_unix_timestamp_us() -> i64 {
            let now = Utc::now();
            now.timestamp_nanos()
        }

        pub fn test() {
            // add your code here
            println!("ms={}", get_unix_timestamp_ms());
            println!("ms={}", get_unix_timestamp_us());
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
mod answer4 {
    use std::time::{SystemTime, UNIX_EPOCH};

    fn get_epoch_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
    pub fn test() {
        println!("ms={}", get_epoch_ms());
    }
}
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    answer4::test();
    _leave!();
}
