
pub struct Port {
    pub index:usize,
    pub data:Vec<isize>
}

pub trait Input {
    fn read(&self) -> isize;
}

impl Input for Port {
    fn read(&self) -> isize {
        let current = self.index;
        self.data[current]
    }
}

// for an ouput, the data is what is supposed to be received.
// the goal is to check algorithm, and the target values
// will be checked by the output
//
// TODO
