pub struct RangePl {
    next: usize,
    end: usize,
}

impl RangePl {
    pub fn new(range: std::ops::Range<usize>) -> Self {
        RangePl {
            next: range.start,
            end: range.end,
        }
    }

    pub fn next_batch(&mut self, target: usize, buffer: &mut [u64]) -> usize {
        self.next = self.next.max(target);
        let start = self.next;
        if start >= self.end {
            return 0;
        }
        let range_len = (self.end - self.next) as usize;
        let len = range_len.min(buffer.len());

        for i in 0..len {
            buffer[i] = self.next as u64;
            self.next += 1;
        }

        len
  }
}
