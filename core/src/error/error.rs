use core::fmt::{Debug, Display};

/// Common interface for all B3 errors.
pub trait Error: Debug + Display {}

impl<E> Error for E where E: Error + Debug {}
