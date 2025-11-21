use std::collections::HashMap;
use std::str::FromStr;

pub struct TowelDesigns {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl TowelDesigns {
    const fn new(patterns: Vec<String>, designs: Vec<String>) -> Self {
        Self { patterns, designs }
    }

    #[must_use]
    pub fn count_possible_designs(&self) -> usize {
        let mut cache = Self::create_cache();
        self.designs
            .iter()
            .filter(|design| self.design_options(design, &mut cache) > 0)
            .count()
    }

    #[must_use]
    pub fn count_total_design_options(&self) -> u64 {
        let mut cache = Self::create_cache();
        self.designs
            .iter()
            .map(|design| self.design_options(design, &mut cache))
            .sum()
    }

    fn create_cache() -> HashMap<String, u64> {
        let mut cache = HashMap::new();
        cache.insert(String::new(), 1);
        cache
    }

    fn design_options(&self, design: &str, cache: &mut HashMap<String, u64>) -> u64 {
        if let Some(&result) = cache.get(design) {
            return result;
        }
        let result = self
            .patterns
            .iter()
            .filter_map(|pattern| design.strip_prefix(pattern))
            .map(|design| self.design_options(design, cache))
            .sum();
        cache.insert(design.to_string(), result);
        result
    }
}

impl FromStr for TowelDesigns {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (patterns, designs) = s.trim().split_once("\n\n").unwrap();
        let patterns = patterns.split(", ").map(String::from).collect();
        let designs = designs.lines().map(String::from).collect();
        Ok(Self::new(patterns, designs))
    }
}
