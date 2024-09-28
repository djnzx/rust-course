fn main () {
    let num: Option<i32> = Some (4) ;
    let split: i32 = 5;
    match num {
        Some (x) => assert! (x < split),
        Some (x) => assert! (x >= split) ,
        None => (),
    }
    println! ("Success!");
}