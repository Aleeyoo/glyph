//! Kill ring — ring buffer for killed/yanked text.

const MAX_ENTRIES: usize = 32;

pub struct KillRing {
    entries: Vec<String>,
    current: usize,
}

impl KillRing {
    pub fn new() -> Self {
        Self { entries: Vec::new(), current: 0 }
    }

    pub fn push(&mut self, text: &str, merge: bool) {
        if merge && !self.entries.is_empty() {
            self.entries.last_mut().unwrap().push_str(text);
        } else {
            if self.entries.len() >= MAX_ENTRIES {
                self.entries.remove(0);
            }
            self.entries.push(text.to_string());
            self.current = self.entries.len() - 1;
        }
    }

    pub fn yank(&self) -> Option<&str> {
        self.entries.last().map(|s| s.as_str())
    }

    pub fn yank_pop(&mut self) -> Option<&str> {
        if self.entries.is_empty() { return None; }
        self.current = self.current.wrapping_sub(1);
        Some(self.entries[self.current % self.entries.len()].as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ring_empty() {
        let kr = KillRing::new();
        assert!(kr.yank().is_none());
    }

    #[test]
    fn push_and_yank() {
        let mut kr = KillRing::new();
        kr.push("hello", false);
        assert_eq!(kr.yank(), Some("hello"));
    }

    #[test]
    fn merge_consecutive() {
        let mut kr = KillRing::new();
        kr.push("hel", false);
        kr.push("lo", true);
        assert_eq!(kr.yank(), Some("hello"));
    }

    #[test]
    fn wrap_at_max() {
        let mut kr = KillRing::new();
        for i in 0..33 {
            kr.push(&i.to_string(), false);
        }
        assert!(kr.yank().is_some());
    }
}
