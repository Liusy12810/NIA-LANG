

pub struct Stack {
    buffer: [usize;100],
    top: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            buffer: [0; 100],
            top: 1,
        }
    }

    pub fn push(&mut self, val: usize) {
        self.buffer[self.top] = val;
        self.top += 1;
    }

    pub fn pop(&mut self) -> usize {
        self.top -= 1;
        self.buffer[self.top]
    }
}
