
mod tis;
use tis::*; // our lib "objects"

/** problem description
 * we have 2 inputs: in.x and in.a
 * we have 2 outputs: out.x and out.a
 * copy data from one input to the correponding output
 */

fn main() {

    // basice struct creation
    let in_a = Input::new(vec![5, 8, 3, -1, 4]);
    let out_a = Output::new(vec![5, 8, 3, -1, 4]);

    // and first use
    let mut data = in_a.read();
    println!("in_a data: {}", data);
    println!("out_b {} check: {}", data, out_a.write(data));

}
