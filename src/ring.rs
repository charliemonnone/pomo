pub struct RingBuffer<T, const N: usize> {
    count: usize,
    next_item: usize,
    data: [T; N],
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new(data: [T; N]) -> Self {
        RingBuffer {
            count: data.len(),
            next_item: 0,
            data,
        }
    }
    pub fn next(&mut self) -> &T {
        let next = &self.data[self.next_item];
        self.next_item = (self.next_item + 1) % self.count;

        next
    }

    pub fn next_mut(&mut self) -> &mut T {
        let next = &mut self.data[self.next_item];
        self.next_item = (self.next_item + 1) % self.count;

        next
    }
}
