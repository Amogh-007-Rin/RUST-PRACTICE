//! Module 056: solution — the reference implementation.

#[derive(Debug, Clone, PartialEq)]
pub struct CowStr(Option<String>);

impl CowStr {
    pub fn new() -> Self {
        CowStr(None)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        CowStr(Some(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        match &self.0 {
            Some(s) => s.as_str(),
            None => "",
        }
    }

    pub fn to_mut(&mut self, source: &str) -> &mut String {
        if self.0.is_none() {
            self.0 = Some(source.to_string());
        }
        self.0.as_mut().unwrap()
    }

    pub fn into_string(self) -> String {
        self.0.unwrap_or_default()
    }
}

impl Default for CowStr {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ArrayBuffer with const generics
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct ArrayBuffer<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T: Default + Copy, const N: usize> ArrayBuffer<T, N> {
    pub fn new() -> Self {
        ArrayBuffer {
            data: [T::default(); N],
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.len >= N {
            return Err("full");
        }
        self.data[self.len] = value;
        self.len += 1;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: Default + Copy, const N: usize> Default for ArrayBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// longest_str
// ---------------------------------------------------------------------------

pub fn longest_str(a: &str, b: CowStr) -> CowStr {
    if a.len() > b.as_str().len() {
        CowStr::from_str(a)
    } else {
        b
    }
}
