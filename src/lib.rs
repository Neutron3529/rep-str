//! This is a crate for caching a repeat pattern of a str
//! with only one allocation by generate a new `RepStr` struct
//! this crate provides.
//! # Examples
//! ```rust
//! let repstr = crate::RepStr::new("#", 50); // generate a RepStr object with max repeat time 50
//! assert!(format!("{empty:#>width$}", empty = "", width = 20) == repstr.repeat_unwrap(20))
//! ```

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let repstr = crate::RepStr::new("#", 50);
        assert!(format!("{empty:#>width$}", empty = "", width = 20) == repstr.repeat_unwrap(20))
    }
}
/// The cache result for a repeatable string.
pub struct RepStr {
    cache: String,
    orig_len: usize,
}
impl RepStr {
    pub fn new(s: &str, max_repeat_time: usize) -> Self {
        Self {
            cache: s.repeat(max_repeat_time),
            orig_len: s.len(),
        }
    }
    #[inline(always)]
    /// This is the function to extract reference of a str with the given repeat time
    /// would panic if the given repeat time is larger than max_repeat_time.
    pub fn repeat_unwrap(&self, repeat: usize) -> &str {
        self.cache.split_at(repeat * self.orig_len).0
    }
    /// This is a basic function to extract reference of a str with the given repeat time
    /// will return Some<&str> if the given repeat time is smaller than max_repeat_time
    /// otherwise return None.
    pub fn repeat(&self, repeat: usize) -> Option<&str> {
        if repeat * self.orig_len <= self.cache.len() {
            Some(self.repeat_unwrap(repeat))
        } else {
            None
        }
    }
}
