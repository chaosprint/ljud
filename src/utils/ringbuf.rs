use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct RingBuf {
    buffer: SmallVec<[f32; 65536 * 2]>,
    start: usize,
    end: usize,
    capacity: usize,
}

impl RingBuf {
    pub fn new(capacity: usize) -> Self {
        RingBuf {
            buffer: SmallVec::with_capacity(capacity),
            start: 0,
            end: 0,
            capacity,
        }
    }

    pub fn push(&mut self, value: f32) -> Option<f32> {
        let evicted_value = if self.buffer.len() < self.capacity {
            self.buffer.push(value);
            self.end += 1;
            None
        } else {
            let evicted = self.buffer[self.start % self.capacity];
            self.buffer[self.end % self.capacity] = value;
            self.end += 1;
            self.start += 1;
            Some(evicted)
        };
        evicted_value
    }

    pub fn pop(&mut self) -> Option<f32> {
        if self.start == self.end {
            None
        } else {
            let value = self.buffer[self.start % self.capacity];
            self.start += 1;
            Some(value)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn as_slice(&self) -> &[f32] {
        if self.start <= self.end {
            &self.buffer[self.start..self.end]
        } else {
            &self.buffer[self.start..self.capacity]
        }
    }
}
