use sysinfo::System;

#[test]
fn main() {
    let mut sys = System::new_all().expect("failed to read system info");
    sys.refresh_all();

    for cpu in sys.cpus() {
        println!("Brand: {}, Freq: {}", cpu.brand(), cpu.frequency(),);
    }
}
