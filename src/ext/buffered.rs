use std::io::{BufReader, Read};

pub trait BufferedRead : Read {
    type BR: Read;
    fn buffered(self) -> Self::BR;
}

impl <R: Read> BufferedRead for R {
    type BR = BufReader<R>;

    fn buffered(self) -> Self::BR {
        BufReader::new(self)
    }
}
