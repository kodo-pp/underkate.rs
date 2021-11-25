use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Handle<Tag> {
    raw: u64,
    _phantom: PhantomData<Tag>,
}

#[derive(Debug, Copy, Clone)]
pub struct HandleGenerator<Tag> {
    counter: u64,
    _phantom: PhantomData<Tag>,
}

impl<Tag> HandleGenerator<Tag> {
    pub fn new() -> Self {
        Self { counter: 0, _phantom: PhantomData }
    }

    pub fn gen_handle(&mut self) -> Handle<Tag> {
        let handle = Handle { raw: self.counter, _phantom: PhantomData };
        self.counter += 1;
        handle
    }
}
