
mod libs; // like an include_path
use libs::{Port, Input}; // then our lib "objects"

/** problem description
 * we have 2 inputs: in.x and in.a
 * we have 2 outputs: out.x and out.a
 * copy data from one input to the correponding output
 */

fn main() {
    // TODO
    println!("TODO");

    // basice struct creation
    let in_a = Port{index: 0, data: vec![5, 8, 3, -1, 4]};

    // and first use
    println!("first in_a data {}", in_a.read());
}
