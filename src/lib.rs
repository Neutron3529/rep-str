//! This is a crate for caching a repeat pattern of a str
//! with only one allocation by generate a new `RepStr` struct
//! this crate provides.
//! # Example 1: Crate RepStr directly.
//! ```rust
//! use rep_str::RepStr;
//! let repstr = rep_str::RepStr::new("#", 50); // generate a RepStr object with max repeat time 50
//! assert!("##########" == repstr.repeat_unwrap(10));
//! assert!("####################" == repstr.repeat_unwrap(20));
//! // no extra allocation would occurs:
//! assert!(repstr.repeat_unwrap(20).as_ptr() == repstr.repeat(12).unwrap().as_ptr())
//! // repstr.repeat_unwrap(51) // panic!
//! ```
//! # Example 2: Crate RepStr by IntoRepStr trait
//! ```rust
//! use rep_str::IntoRepStr;
//! let repstr = "🦀".repeat_cache(20);
//! assert!(Some("🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀") == repstr.repeat(20));
//! assert!(None == repstr.repeat(21));
//! ```
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn rep_str_new_and_repeat_unwrap() {
        let repstr = RepStr::new("#", 50);
        assert!(format!("{empty:#>width$}", empty = "", width = 20) == repstr.repeat_unwrap(20));
        assert!(repstr.repeat_unwrap(20).as_ptr()==repstr.repeat(12).unwrap().as_ptr())
    }
    #[test]
    fn rep_str_into_rep_str_check(){
        let repstr = "#".repeat_cache(20);
        assert!(format!("{empty:#>width$}", empty = "", width = 20) == repstr.repeat(20).unwrap());
        assert!(None == repstr.repeat(21));
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
pub trait IntoRepStr{
    fn repeat_cache(self,repeat:usize)->RepStr;
}
impl IntoRepStr for &str{
    fn repeat_cache(self,repeat:usize)->RepStr{
        RepStr::new(self,repeat)
    }
}

