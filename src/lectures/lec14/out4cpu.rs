use sysinfo::System;

#[test]
fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    for cpu in sys.cpus() {
        println!("Brand: {}, Freq: {}", cpu.brand(), cpu.frequency(),);
    }
}
