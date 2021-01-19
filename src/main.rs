mod gb;

use gb::gb::GB;

fn main() {
    let gb = GB::new();
    gb.emulate_cycle();
}
