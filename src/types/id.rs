use std::num::ParseIntError;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

#[derive(Clone, Ord, PartialEq, PartialOrd, Hash, Serialize, Deserialize, Eq, Default, Debug)]
pub struct ID(pub String);

impl Deref for ID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Into<String>> From<T> for ID {
    fn from(v: T) -> Self {
        ID(v.into())
    }
}

macro_rules! try_from_integers {
    ($($ty:ty),*) => {
        $(
            impl TryFrom<ID> for $ty {
                    type Error = ParseIntError;

                    fn try_from(id: ID) -> Result<Self, Self::Error> {
                        id.0.parse()
                    }
                }
        )*
    };
}

try_from_integers!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);
