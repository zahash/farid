use std::{fmt::Display, ops::Deref};

mod macros;

#[derive(Debug)]
pub struct Id<T>(T);

impl<T> Deref for Id<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Display for Id<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub trait Advance {
    fn advance(&mut self);
}

pub struct Gen<T> {
    id: T,
    free: Vec<T>,
}

impl<T> Gen<T> {
    pub const fn init(start: T) -> Self {
        Self {
            id: start,
            free: vec![],
        }
    }

    pub fn id(&mut self) -> Id<T>
    where
        T: Clone + Advance,
    {
        match self.free.pop() {
            Some(id) => Id(id),
            None => {
                let id = self.id.clone();
                self.id.advance();
                Id(id)
            }
        }
    }

    pub fn free(&mut self, Id(id): Id<T>) {
        self.free.push(id);
    }
}
