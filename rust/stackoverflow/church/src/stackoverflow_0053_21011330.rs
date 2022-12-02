// @filename   : stackoverflow_0053_21011330.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/21011330
// @title      : How do I invoke a system command and capture its output?

#[allow(dead_code)]
mod answer1 {
    /*
    std::process::Command allows for that.

    There are multiple ways to spawn a child process and execute an arbitrary command on the machine:

    spawn — runs the program and returns a value with details
    output — runs the program and returns the output
    status — runs the program and returns the exit code
    One simple example from the docs:

    use std::process::Command;

    Command::new("ls")
            .arg("-l")
            .arg("-a")
            .spawn()
            .expect("ls command failed to start");
    */

    mod code1 {
        use std::process::Command;
        pub fn test() {
            // add your code here
            let output = Command::new("ls")
                .arg("-l")
                .arg("-a")
                .spawn()
                .expect("ls command failed to start");
            println!("{:?}", output);
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
        use std::process::Command;
        pub fn test() {
            // add your code here
            let output = Command::new("/bin/cat")
                .arg("Cargo.toml")
                .output()
                .expect("failed to execute process");

            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

            assert!(output.status.success());
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
