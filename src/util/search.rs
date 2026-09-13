
use suffix::SuffixTable;

pub struct SearchTree {
    search_items: Vec<String>,
    table: SuffixTable<'static, 'static>,
    offsets: Vec<usize>, // start byte offset of each part in the combined text
}

impl SearchTree {
    pub fn new(search_items: Vec<String>) -> Self {
        let mut text = String::new();
        let mut offsets = Vec::with_capacity(search_items.len());

        for p in &search_items {
            offsets.push(text.len());
            text.push_str(p);
            text.push('\0'); // separator guaranteed not to appear in part numbers
        }

        // SuffixTable wants owned/'static data, so leak the text into a 'static str.
        // Fine for a search index built once and kept for the program's lifetime.
        let text: &'static str = Box::leak(text.into_boxed_str());
        let table = SuffixTable::new(text);

        return Self { search_items, table, offsets };
    }

    /// Returns matching part strings, best match first.
    /// "Best" = exact match, then shortest string containing the needle,
    /// then alphabetical for stability.
    pub fn search(&self, needle: &str) -> Vec<&str> {
        let mut idxs: Vec<usize> = self.table.positions(needle)
            .iter()
            .map(|&pos| self.offset_to_index(pos as usize))
            .collect();

        idxs.sort_unstable();
        idxs.dedup();

        let mut results: Vec<&str> = idxs.into_iter()
            .map(|i| self.search_items[i].as_str())
            .collect();

        results.sort_by(|a, b| {
            let a_exact = *a == needle;
            let b_exact = *b == needle;
            b_exact.cmp(&a_exact)          // exact matches first
                .then(a.len().cmp(&b.len())) // then shortest
                .then(a.cmp(b))               // then alphabetical
        });

        results
    }

    fn offset_to_index(&self, pos: usize) -> usize {
        match self.offsets.binary_search(&pos) {
            Ok(i) => i,
            Err(i) => i - 1,
        }
    }
}
