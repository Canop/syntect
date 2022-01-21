use {
    std::fmt,
};

#[derive(Debug, Clone)]
pub struct CrashError {
    message: String,
}

impl CrashError {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self {
            message: s.into(),
        }
    }
}

impl fmt::Display for CrashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntect crashed : {:?}", self.message)
    }
}

#[macro_export]
macro_rules! crash {
    ($($arg:tt)*) => (Err(CrashError::new(&format!($($arg)*))));
}
