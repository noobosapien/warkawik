use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

pub struct SendFn {
    pub send_msg: Arc<Box<dyn Fn(u8, Rc<String>) + Send + Sync>>,
}

impl fmt::Debug for SendFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SendFn")
    }
}

// trait NewTrait: Fn<(Rc<std::string::String>)> + std::marker::Copy {}

impl SendFn {
    pub fn new(send_msg: Arc<Box<dyn Fn(u8, Rc<String>) + Send + Sync>>) -> Self {
        Self { send_msg }
    }

    pub fn get(&self) -> Arc<Box<dyn Fn(u8, Rc<String>) + Send + Sync>> {
        Arc::clone(&self.send_msg)
    }
}
