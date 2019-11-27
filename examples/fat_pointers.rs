// via https://stackoverflow.com/a/57754902
use std::mem::size_of;

trait Animal {
    fn speak(&self);
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

fn main() {
	dbg!(size_of::<&Cat>());
	dbg!(size_of::<&dyn Animal>());

	dbg!(size_of::<&u32>());
	dbg!(size_of::<&[u32; 2]>());
	dbg!(size_of::<&[u32]>());
}