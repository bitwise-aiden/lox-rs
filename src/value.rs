use crate::object::{ObjAllocator, ObjRef};

#[derive(Clone, Copy, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(ObjRef<String>),
}

impl Value {
    pub fn print(&self, allocator: &ObjAllocator) -> () {
        match self {
            Value::Nil => print!("nil"),
            Value::Bool(value) => print!("{value}"),
            Value::Number(value) => print!("{value}"),
            Value::String(reference) => print!("{}", allocator.deref(*reference)),
        }
    }

    pub fn is_falsy(&self) -> bool {
        match self {
            Value::Bool(value) => !value,
            Value::Nil => true,
            _ => false,
        }
    }
}
