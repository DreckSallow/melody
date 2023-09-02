#[derive(Debug)]
pub enum Condition {
    True,
    False,
}

impl<T> From<Option<T>> for Condition {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(_) => Condition::True,
            None => Condition::False,
        }
    }
}

impl From<bool> for Condition {
    fn from(value: bool) -> Self {
        match value {
            true => Condition::True,
            false => Condition::False,
        }
    }
}

#[macro_export]
macro_rules! select {
    ($expression:expr,$true:expr,$false:expr) => {
        match Into::<Condition>::into($expression) {
            Condition::True => $true,
            Condition::False => $false,
        }
    };
}
