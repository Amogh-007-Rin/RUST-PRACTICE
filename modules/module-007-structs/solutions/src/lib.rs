//! Module 007: solution — the reference implementation.

/// A book in the library.
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
}

impl Book {
    /// Creates a new `Book` from its fields.
    pub fn new(title: &str, author: &str, pages: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
        }
    }

    /// Returns a one-line description like `"Title" by Author (NNN pages)`.
    pub fn summary(&self) -> String {
        format!(
            "\"{}\" by {} ({} pages)",
            self.title, self.author, self.pages
        )
    }

    /// Returns `true` if the book is longer than 400 pages.
    pub fn is_long(&self) -> bool {
        self.pages > 400
    }
}

/// A point in 2D space (a tuple struct with two `f64`s).
pub struct Point(pub f64, pub f64);

impl Point {
    /// Returns the Euclidean distance between this point and `other`.
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = other.0 - self.0;
        let dy = other.1 - self.1;
        (dx * dx + dy * dy).sqrt()
    }
}
