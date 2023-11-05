use std::process::Command;
use std::os::unix::process::CommandExt;
use std::path::Path;

const SIZE: usize = 5;

fn main() {
    let mut nums = [0, 1, 2, 3, 4];

	let child_path = Path::new("/mnt/c/COS3105/rustOnFork/src");
	let mut child = Command::new(child_path)
		.spawn()
		.expect("Failed to execute child process");


    let status = child.wait().expect("Failed to wait for child process");

    if status.success() {
        for i in 0..SIZE {
            nums[i] = (nums[i] as i32) * -1;
            println!("CHILD: {}", nums[i]);
        }
    } else {
        eprintln!("Child process failed");
    }

    for i in 0..SIZE {
        println!("PARENT: {}", nums[i]);
    }
}
