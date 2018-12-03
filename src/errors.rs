use std::io;
use std::fmt::Display;
use disks::DiskError;

pub trait IoContext<T> {
    fn with_context<F: FnMut(Box<Display>) -> String>(self, func: F) -> io::Result<T>;
}

impl<T> IoContext<T> for io::Result<T> {
    fn with_context<F: FnMut(Box<Display>) -> String>(self, mut func: F) -> io::Result<T> {
        self.map_err(|why| io::Error::new(why.kind(), func(Box::new(why))))
    }
}

// NOTE: This can be removed once RFC #1210 is implemented.
impl<T> IoContext<T> for Result<T, DiskError> {
    fn with_context<F: FnMut(Box<Display>) -> String>(self, mut func: F) -> io::Result<T> {
        self.map_err(|why| io::Error::new(io::ErrorKind::Other, func(Box::new(why))))
    }
}

// Requires RFC #1210: https://github.com/rust-lang/rust/issues/37653
// default impl<T, E: Display> IoContext<T> for Result<T, E> {
//     fn with_context<F: FnMut(Box<Display>) -> String>(self, mut func: F) -> io::Result<T> {
//         self.map_err(|why| io::Error::new(io::ErrorKind::Other, func(Box::new(why))))
//     }
// }
