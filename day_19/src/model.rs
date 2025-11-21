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
        let mut cache = self.create_cache();
        self.designs
            .iter()
            .filter(|design| self.validate_design(design, &mut cache))
            .count()
    }

    fn create_cache(&self) -> HashMap<String, bool> {
        let mut cache = HashMap::new();
        for pattern in &self.patterns {
            cache.insert(pattern.clone(), true);
        }
        cache
    }

    fn validate_design(&self, design: &str, cache: &mut HashMap<String, bool>) -> bool {
        if let Some(&result) = cache.get(design) {
            return result;
        }
        let result = self
            .patterns
            .iter()
            .filter_map(|pattern| design.strip_prefix(pattern))
            .any(|design| self.validate_design(design, cache));
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
