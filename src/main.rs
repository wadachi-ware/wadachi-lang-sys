use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // read arguments
    let args: Vec<String> = env::args().collect();

    // read program from file
    let filename = &args[1];
    let mut file = File::open(filename).expect("file not found");
    let mut program = String::new();
    file.read_to_string(&mut program).expect("cannot read file contents");

    /*
        int main(void) {
            return 0;
        }
    */
    println!("main:");
    println!("addi  sp, sp, -16");
    println!("sw    s0, 12(sp)");
    println!("addi  s0, sp, 16");
    println!("li    a5, 0");
    println!("mv    a0, a5");
    println!("lw    s0, 12(sp)");
    println!("addi  sp, sp, 16");
    println!("jr    ra");
}
