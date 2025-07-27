//! # sringbuf
//!
//! `sringbuf` is a small ring buffer implementation
#[derive(Clone, Debug, PartialEq)]
pub struct RingBuffer<T, const N: usize> {
    contents: [Option<T>; N],
    read_index: usize,
    write_index: usize,
}

impl<T, const N: usize> RingBuffer<T, N> 
where
    T: Copy
{
    /// Creates a new RingBuffer instance
    ///
    /// # Examples
    /// 
    /// ```
    /// let ring_buffer: sringbuf::RingBuffer<u8, 5> = sringbuf::RingBuffer::new();
    ///
    /// const num: usize = 5;
    /// let ring_buffer: sringbuf::RingBuffer<char, num> = sringbuf::RingBuffer::new();
    /// ```
    pub const fn new() -> RingBuffer<T, N> {
        assert!(N > 0);

        RingBuffer {
            contents: [None; N],
            read_index: 0,
            write_index: 0,
        }
    }

    /// Writes a value to the beginning of a ring buffer
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ring_buffer: sringbuf::RingBuffer<u8, 5> = sringbuf::RingBuffer::new();
    /// ring_buffer.write(1);
    /// ```
    pub fn write(&mut self, data: T) {
        self.contents[self.write_index] = Some(data);

        if self.write_index + 1 == N {
            self.write_index = 0;
            return;
        }

        self.write_index += 1;
    }

    /// Reads the oldest available element a ring buffer
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ring_buffer: sringbuf::RingBuffer<u8, 5> = sringbuf::RingBuffer::new();
    /// ring_buffer.write(1);
    /// let data = ring_buffer.read();
    pub fn read(&mut self) -> Option<T> {
        let data = self.contents[self.read_index];
        match data {
            Some(_) => {
                self.contents[self.read_index] = None;
                if self.read_index + 1 == N {
                    self.read_index = 0;
                    return data;
                }

                self.read_index += 1;
                data
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid_inputs() {
        let ring_buffer: RingBuffer<char, 5> = RingBuffer::new();

        assert_eq!(ring_buffer, RingBuffer {
            contents: [None; 5],
            read_index: 0,
            write_index: 0,
        });
    }

    #[test]
    #[should_panic]
    fn new_invalid_input() {
        let _ring_buffer: RingBuffer<char, 0> = RingBuffer::new();
    }

    #[test]
    fn write_valid_inputs_first() {
        let mut ring_buffer: RingBuffer<char, 5> = RingBuffer::new();

        ring_buffer.write('a');
        
        assert_eq!(ring_buffer, RingBuffer {
            contents: [Some('a'), None, None, None, None],
            read_index: 0,
            write_index: 1,
        });
    }

    #[test]
    fn write_wrap_around() {
        let mut ring_buffer: RingBuffer<u8, 5> = RingBuffer::new();

        ring_buffer.write(1);
        ring_buffer.write(2);
        ring_buffer.write(3);
        ring_buffer.write(4);
        ring_buffer.write(5);
        ring_buffer.write(6);
        ring_buffer.write(7);

        assert_eq!(ring_buffer, RingBuffer {
            contents: [Some(6), Some(7), Some(3), Some(4), Some(5)],
            read_index: 0,
            write_index: 2,
        });
    }

    #[test]
    fn read_first() {
        let mut ring_buffer: RingBuffer<u8, 3> = RingBuffer::new();

        ring_buffer.write(1);

        assert_eq!(ring_buffer.read(), Some(1));
        assert_eq!(ring_buffer, RingBuffer {
            contents: [None; 3],
            read_index: 1,
            write_index: 1,
        });
    }

    #[test]
    fn read_empty() {
        let mut ring_buffer: RingBuffer<u8, 3> = RingBuffer::new();

        assert_eq!(ring_buffer.read(), None);
        assert_eq!(ring_buffer, RingBuffer {
            contents: [None; 3],
            read_index: 0,
            write_index: 0,
        });
    }

    fn read_wrap_around() {
        let mut ring_buffer: RingBuffer<u8, 3> = RingBuffer::new();

        ring_buffer.write(1);
        ring_buffer.write(2);
        ring_buffer.write(3);
        ring_buffer.write(4);
        ring_buffer.write(5);
        ring_buffer.write(6);

        ring_buffer.read();

        assert_eq!(ring_buffer.read(), Some(5));
        assert_eq!(ring_buffer, RingBuffer {
            contents: [None, None, Some(6)],
            read_index: 2,
            write_index: 0,
        });
    }
}
