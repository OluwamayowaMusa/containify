use sys_mount::Unmount;
use sys_mount::UnmountFlags;
use std::process::Command;
use sys_mount::Mount;
use std::env;
use std::os::unix::fs;
use rustix::thread::{unshare, UnshareFlags};

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).expect("No argument passed");

    println!("Argument passed: {}", arg);

    match arg.as_str() {
        "run" => {
            println!{"Runnning run"};
            run();
        },
        "child" => {
            println!("Running child");
            child();
        },
        _ => panic!("No command matched: {}", arg)
    }
}

fn run() {
    let mut cmd = Command::new("/proc/self/exe");
    cmd.arg("child");

    match unshare(UnshareFlags::NEWUTS | UnshareFlags::NEWPID | UnshareFlags::NEWNS) {
	Ok(_) => {
	    println!("PID, UTS and MOUNT Namespace set up");

	    match cmd.status() {
	        Ok(result) => println!("Okay in run, {}", result),
		Err(e) => println!("Error in run, {}", e),
	    }
	},
	Err(e) => println!("Error, unable to set up namespcae: {}", e),
    }
}

fn child() {
    match fs::chroot("/home/vagrant/containify/fs_container/ubuntu/") {
        Ok(_) => {
            println!("Root changed successfully");
            match env::set_current_dir("/") {
                Ok(_) => println!("Changed current directory to root"),
                Err(e) => println!("Unable to change directory: {e}"),
            }
        },
        Err(e) => println!("Error occured: {}", e),

    }

    let mut change_hostname = Command::new("hostname");
    change_hostname.arg("container");
    change_hostname.status().unwrap();

    let mount_proc = Mount::builder()
        .fstype("proc")
        .mount("proc", "/proc");

    match mount_proc {
        Ok(mount) => {
            println!("Mount proc successfully");
            let _mount = mount.into_unmount_drop(UnmountFlags::DETACH);
            
            let mut cmd = Command::new("bash");

            match cmd.status() {
                Ok(result) => println!("Okay in child, {}", result),
                Err(error) => println!("Error in child, {}", error),
            }
        },
        Err(error) => {
            println!("Unable to mount: {:?}", error);
        },
    }
}
