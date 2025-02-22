use sysinfo::System;

#[test]
fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    for component in sys.cpus() {
        println!(
            "Датчик: {}, Температура: {}°C",
            component.brand(),
            component.name()
        );
    }
}
