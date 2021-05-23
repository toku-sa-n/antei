use crate::status;

pub struct Error<T> {
    status: status::NotSuccess,
    value: T,
}
impl<T> Error<T> {
    pub fn status(&self) -> &status::NotSuccess {
        &self.status
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}
