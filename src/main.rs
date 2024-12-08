use libphext::phext;

fn main() {
    let address = phext::to_coordinate("1.1.1/1.1.1/1.1.1");
    println!("Mini 64K Online - there's no place like {}", address);
}
