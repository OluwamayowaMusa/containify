use sys_mount::Unmount;
use sys_mount::UnmountFlags;
use unshare::Command;
use unshare::Namespace;
use sys_mount::Mount;
use sys_mount::MountFlags;
use std::env;
use std::os::unix::fs;
use std::thread;
use nix::sched::{unshare, CloneFlags};

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
        _ => panic!("No commad matched: {}", arg)
    }
}

fn run() {
    let namespaces = [Namespace::Uts, Namespace::Pid, Namespace::Mount];
    let mut cmd = Command::new("/proc/self/exe");
    cmd.unshare(&namespaces);
    cmd.arg("child");

    match cmd.status() {
        Ok(result) => println!("Okay, {}", result),
        Err(error) => println!("Error, {}", error),
    }
}

fn child() {
    match unshare(CloneFlags::CLONE_NEWNS) {
	Ok(_) => {
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
	
	    let mut change_hostname = Command::new("/bin/hostname");
	    change_hostname.arg("container");
	    change_hostname.status().unwrap();
	
	    let mount_proc = Mount::builder()
	        .fstype("proc")
	        //.flags(MountFlags::from_bits(0x040000_u64).unwrap())
	        .mount("proc", "/proc");
	
	    match mount_proc {
	        Ok(mount) => {
	            println!("Mount proc successfully");
	            let _mount = mount.into_unmount_drop(UnmountFlags::DETACH);
	            
	            let mut cmd = Command::new("/bin/bash");
	
	            match cmd.status() {
	                Ok(result) => println!("Okay in child, {}", result),
	                Err(error) => println!("Error in child, {}", error),
	            }
	        },
	        Err(error) => {
	            println!("Unable to mount: {:?}", error);
	        },
	    }

	},
	Err(error) => println!("Error, {}", error)
    }
}


//fn main() {
//
//    match unshare(CloneFlags::CLONE_NEWNS) {
//	Ok(_) => {
//	    match fs::chroot("/home/vagrant/containify/fs_container/ubuntu/") {
//	        Ok(_) => {
//	            println!("Root changed successfully");
//	            match env::set_current_dir("/") {
//	                Ok(_) => println!("Changed current directory to root"),
//	                Err(e) => println!("Unable to change directory: {e}"),
//	            }
//	        },
//	        Err(e) => println!("Error occured: {}", e),
//	
//	    }
//
//	    let mount_proc = Mount::builder()
//	        .fstype("proc")
//	        .mount("proc", "/proc");
//	
//	    match mount_proc {
//	        Ok(mount) => {
//	            println!("Mount proc successfully");
//	            let _mount = mount.into_unmount_drop(UnmountFlags::DETACH);
//	            
//	            let mut cmd = Command::new("/bin/bash");
//	
//	            match cmd.status() {
//	                Ok(result) => println!("Okay in child, {}", result),
//	                Err(error) => println!("Error in child, {}", error),
//	            }
//	        },
//	        Err(error) => {
//	            println!("Unable to mount: {:?}", error);
//	        },
//	    }
//
//	},
//	Err(error) => println!("Error: {}", error),
//    }
//
//}
