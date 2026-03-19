/// English RustDoc comment.
/// Represents a paginated text document navigator.
#[derive(Debug, Default)]
pub struct DocumentPager {
    pages: Vec<Vec<String>>,
    current_page: usize,
}

impl DocumentPager {
    /// English RustDoc comment.
    /// Creates a new pager from a collection of pages.
    pub fn new(pages: Vec<Vec<String>>) -> Self {
        Self {
            pages,
            current_page: 0,
        }
    }

    /// English RustDoc comment.
    /// Returns true if the pager contains no pages.
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// English RustDoc comment.
    /// Returns the total number of pages.
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// English RustDoc comment.
    /// Returns the current page number using 1-based indexing.
    pub fn current_page_number(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.current_page + 1
        }
    }

    /// English RustDoc comment.
    /// Returns the current page as a slice of lines.
    pub fn current_page(&self) -> &[String] {
        if self.is_empty() {
            &[]
        } else {
            &self.pages[self.current_page]
        }
    }

    /// English RustDoc comment.
    /// Moves to the next page if possible.
    pub fn next(&mut self) {
        if self.current_page + 1 < self.pages.len() {
            self.current_page += 1;
        }
    }

    /// English RustDoc comment.
    /// Moves to the previous page if possible.
    pub fn previous(&mut self) {
        self.current_page = self.current_page.saturating_sub(1);
    }

    /// English RustDoc comment.
    /// Returns the current page indicator as `(current, total)`.
    pub fn indicator(&self) -> (usize, usize) {
        (self.current_page_number(), self.page_count())
    }
}

#[cfg(test)]
mod tests {
    use super::DocumentPager;

    /// English RustDoc comment.
    /// Verifies that an empty pager reports zero pages.
    #[test]
    fn empty_pager() {
        let pager = DocumentPager::new(Vec::new());

        assert!(pager.is_empty());
        assert_eq!(pager.page_count(), 0);
        assert_eq!(pager.current_page_number(), 0);
        assert!(pager.current_page().is_empty());
        assert_eq!(pager.indicator(), (0, 0));
    }

    /// English RustDoc comment.
    /// Verifies that a pager starts from the first page.
    #[test]
    fn pager_starts_from_first_page() {
        let pager = DocumentPager::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string()],
        ]);

        assert!(!pager.is_empty());
        assert_eq!(pager.page_count(), 2);
        assert_eq!(pager.current_page_number(), 1);
        assert_eq!(
            pager.current_page(),
            &vec!["A".to_string(), "B".to_string()]
        );
        assert_eq!(pager.indicator(), (1, 2));
    }

    /// English RustDoc comment.
    /// Verifies that moving forward stops at the last page.
    #[test]
    fn pager_next_stops_at_last_page() {
        let mut pager = DocumentPager::new(vec![vec!["A".to_string()], vec!["B".to_string()]]);

        pager.next();
        assert_eq!(pager.current_page_number(), 2);
        assert_eq!(pager.current_page(), &vec!["B".to_string()]);

        pager.next();
        assert_eq!(pager.current_page_number(), 2);
        assert_eq!(pager.current_page(), &vec!["B".to_string()]);
    }

    /// English RustDoc comment.
    /// Verifies that moving backward stops at the first page.
    #[test]
    fn pager_previous_stops_at_first_page() {
        let mut pager = DocumentPager::new(vec![vec!["A".to_string()], vec!["B".to_string()]]);

        pager.next();
        assert_eq!(pager.current_page_number(), 2);

        pager.previous();
        assert_eq!(pager.current_page_number(), 1);
        assert_eq!(pager.current_page(), &vec!["A".to_string()]);

        pager.previous();
        assert_eq!(pager.current_page_number(), 1);
        assert_eq!(pager.current_page(), &vec!["A".to_string()]);
    }
}
