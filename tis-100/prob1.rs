
mod tis;
use tis::{Port, Input}; // our lib "objects"

/** problem description
 * we have 2 inputs: in.x and in.a
 * we have 2 outputs: out.x and out.a
 * copy data from one input to the correponding output
 */

fn main() {

    // basice struct creation
    let in_a = Port{index: 0, data: vec![5, 8, 3, -1, 4]};

    // and first use
    println!("first in_a data {}", in_a.read());
}
