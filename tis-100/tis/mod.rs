
struct Port {
	pub index:usize,
	pub data:Vec<isize>
}

pub struct Input {
    port: Port
}

pub struct Output {
    port: Port
}

// input implementation
impl Input {

    // constructor/factory?
    pub fn new(data_:Vec<isize>) -> Input {
        Input {
            port: Port {
                index: 0, data: data_
            }
        }
    }

    // data reader
	pub fn read(&self) -> isize {
		let current = self.port.index;
		self.port.data[current]
	}
}

// input implementation
impl Output {

    // constructor/factory?
    pub fn new(data_:Vec<isize>) -> Output {
        Output {
            port: Port {
                index: 0,
                data: data_
            }
        }
    }

    // data writer/checker
	pub fn write(&self, data_:isize) -> bool {
		let current = self.port.index;
        // TODO increment the index to be ready for the next call
		data_ == self.port.data[current]
	}
}
