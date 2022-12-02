// @filename   : stackoverflow_0017_27791532.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27791532
// @title      : How do I create a global, mutable singleton?

#[allow(dead_code)]
mod answer1 {
    // If you remove the Mutex then you have a global singleton without any mutability.
    // You can also use a RwLock instead of a Mutex to allow multiple concurrent readers.
    mod code1 {
        // Using lazy-static

        use lazy_static::lazy_static; // 1.4.0
        use std::sync::Mutex;

        lazy_static! {
            static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
        }

        fn do_a_call() {
            ARRAY.lock().unwrap().push(1);
        }
        pub fn test() {
            // add your code here
            do_a_call();
            do_a_call();
            do_a_call();

            println!("called {}", ARRAY.lock().unwrap().len());
        }
    }
    mod code2 {
        // Using once_cell

        use once_cell::sync::Lazy; // 1.3.1
        use std::sync::Mutex;

        static ARRAY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![]));

        fn do_a_call() {
            ARRAY.lock().unwrap().push(1);
        }
        pub fn test() {
            // add your code here
            do_a_call();
            do_a_call();
            do_a_call();

            println!("called {}", ARRAY.lock().unwrap().len());
        }
    }
    mod code3 {
        // Using std::sync::SyncLazy
        //#![feature(once_cell)] // 1.53.0-nightly (2021-04-01 d474075a8f28ae9a410e)
        //use std::lazy::SyncLazy;
        //use std::sync::Mutex;
        //static ARRAY: SyncLazy<Mutex<Vec<u8>>> = SyncLazy::new(|| Mutex::new(vec![]));

        fn do_a_call() {
            //ARRAY.lock().unwrap().push(1);
        }
        pub fn test() {
            // add your code here
            do_a_call();
            do_a_call();
            do_a_call();

            // println!("called {}", ARRAY.lock().unwrap().len());
        }
    }
    mod code4 {
        // A special case: atomics
        // If you only need to track an integer value, you can directly use an atomic:

        use std::sync::atomic::{AtomicUsize, Ordering};

        static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

        fn do_a_call() {
            CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        }

        pub fn test() {
            do_a_call();
            do_a_call();
            do_a_call();

            println!("called {}", CALL_COUNT.load(Ordering::SeqCst));
        }
    }
    mod code5 {
        // Manual, dependency-free implementation

        use std::sync::{Mutex, Once};
        use std::time::Duration;
        use std::{mem::MaybeUninit, thread};

        struct SingletonReader {
            // Since we will be used in many threads, we need to protect
            // concurrent access
            inner: Mutex<u8>,
        }

        fn singleton() -> &'static SingletonReader {
            // Create an uninitialized static
            static mut SINGLETON: MaybeUninit<SingletonReader> = MaybeUninit::uninit();
            static ONCE: Once = Once::new();

            unsafe {
                ONCE.call_once(|| {
                    // Make it
                    let singleton = SingletonReader {
                        inner: Mutex::new(0),
                    };
                    // Store it to the static var, i.e. initialize it
                    SINGLETON.write(singleton);
                });

                // Now we give out a shared reference to the data, which is safe to use
                // concurrently.
                SINGLETON.assume_init_ref()
            }
        }

        //#[warn(clippy::needless_collect)]
        pub fn test() {
            // Let's use the singleton in a few threads
            let threads: Vec<_> = (0..10)
                .map(|i| {
                    thread::spawn(move || {
                        thread::sleep(Duration::from_millis(i * 10));
                        let s = singleton();
                        let mut data = s.inner.lock().unwrap();
                        *data = i as u8;
                    })
                })
                .collect();
            println!("{:?}", threads);

            // And let's check the singleton every so often
            for _ in 0u8..20 {
                thread::sleep(Duration::from_millis(5));

                let s = singleton();
                let data = s.inner.lock().unwrap();
                println!("It is: {}", *data);
            }

            for thread in threads.into_iter() {
                thread.join().unwrap();
            }
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
        code4::test();
        code5::test();
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
