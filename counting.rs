#![feature(portable_simd)]

use std::simd::u64x8;

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
        const PROGRESSION: u64x8 = u64x8::from_array([0, 1, 2, 3, 4, 5, 6, 7]);
        const LANES: usize = PROGRESSION.lanes();
        const SPEC_LENGTH: usize = LANES * 2;
        if buffer.len() == SPEC_LENGTH && (self.next + SPEC_LENGTH) <= self.end {
            // Specialization for 16 elements
            let low = u64x8::splat((self.next + target) as u64) + PROGRESSION;
            buffer[..LANES].copy_from_slice(low.as_array());

            let high = low + u64x8::splat(LANES as u64);
            buffer[LANES..].copy_from_slice(high.as_array());

            self.next += SPEC_LENGTH;

            SPEC_LENGTH
        } else {
            // General case for slice of any size
            let range_len = self.end - self.next;
            let len = range_len.min(buffer.len());

            for chunk in buffer[..len].chunks_mut(8) {
                // This code duplication is required for compiler to vectorize code
                let len = chunk.len();
                for (item, offset) in chunk.iter_mut().zip(0..len) {
                    *item = (self.next + offset) as u64;
                }

                self.next += len;
            }

            len
        }
    }
}
