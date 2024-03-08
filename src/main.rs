use sysinfo::{Networks, System};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=> System: {:?}: {:?}", System::name(), System::host_name());
}