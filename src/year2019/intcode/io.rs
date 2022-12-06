use std::{cell::RefCell, rc::Rc};

use super::{Error, Word};

#[derive(Debug, Clone)]
struct ChannelData<W: Word> {
    buf: Vec<W>,
    max_len: Option<usize>,
}

impl<W: Word> ChannelData<W> {
    fn new(max_len: Option<usize>) -> Self {
        Self {
            buf: Vec::new(),
            max_len,
        }
    }
}

impl<W: Word> ChannelData<W> {
    pub fn is_full(&self) -> bool {
        match self.max_len {
            None => false,
            Some(max_len) => self.buf.len() >= max_len,
        }
    }
}

#[derive(Debug, Clone)]
enum ChannelInner<W: Word> {
    Unique(ChannelData<W>),
    Shared(Rc<RefCell<ChannelData<W>>>),
}

impl<W: Word> ChannelInner<W> {
    fn call_mut<T>(&mut self, f: impl FnOnce(&mut ChannelData<W>) -> T) -> T {
        match self {
            ChannelInner::Unique(data) => f(data),
            ChannelInner::Shared(data) => f(&mut *data.borrow_mut()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Channel<W: Word> {
    inner: ChannelInner<W>,
    pub channel_name: String,
}

impl<W: Word> Default for Channel<W> {
    fn default() -> Self {
        Self::new_unique(None)
    }
}

impl<W: Word> Channel<W> {
    pub fn new_unique(max_len: Option<usize>) -> Self {
        Channel {
            inner: ChannelInner::Unique(ChannelData::new(max_len)),
            channel_name: String::new(),
        }
    }

    pub fn new_shared(max_len: Option<usize>) -> Self {
        Channel {
            inner: ChannelInner::Shared(Rc::new(RefCell::new(ChannelData::new(max_len)))),
            channel_name: String::new(),
        }
    }

    pub fn read(&mut self) -> Result<W, Error<W>> {
        self.inner.call_mut(|data| {
            if data.buf.is_empty() {
                Err(Error::EndOfInput)
            } else {
                Ok(data.buf.remove(0))
            }
        })
    }

    pub fn iter(&mut self) -> impl Iterator<Item = W> + '_ {
        std::iter::repeat_with(|| self.read())
            .take_while(Result::is_ok)
            .map(Result::unwrap)
    }

    pub fn write(&mut self, val: W) -> Result<(), Error<W>> {
        self.inner.call_mut(|data| {
            if data.is_full() {
                return Err(Error::OutputBufferOverflow {
                    max_len: data.max_len.unwrap(),
                });
            }
            data.buf.push(val);
            Ok(())
        })
    }

    pub fn clear(&mut self) {
        self.inner.call_mut(|data| data.buf.clear());
    }
}
