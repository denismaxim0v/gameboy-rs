mod gb;

use gb::gb::GB;

fn main() {
    let mut gb = GB::new();
    let quit = 0;
    while quit == 0{
        gb.emulate_cycle();
    }
}
