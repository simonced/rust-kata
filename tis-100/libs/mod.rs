
struct Port {
    index:usize,
    data:Vec<isize>
}

pub struct Input {
    port: Port
}

pub struct Output {
    port: Port
}


impl Input {
    // constructor
    pub fn new(data_:Vec<isize>) -> Input {
        Input{ port: Port{index: 0, data: data_} }
    }

    pub fn read_next(&self) -> isize {
        // pointer
        let current:usize = self.port.index+1;

        // next pointer position (how to change that unmutable!?)
        self.port.index = self.port.index + 1;
        // TODO deal with last data

        self.port.data[current]
    }
}

// for an ouput, the data is what is supposed to be received.
// the goal is to check algorithm, and the target values
// will be checked by the output
//
// TODO
