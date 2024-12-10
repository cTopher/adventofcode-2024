use std::cmp::Ordering;
use std::str::FromStr;

pub struct SleighLaunchSafetyManual {
    page_ordering_rules: Vec<(u32, u32)>,
    pages_per_update: Vec<Vec<u32>>,
}

impl SleighLaunchSafetyManual {
    fn pages_are_ordered(&self, pages: &[u32]) -> bool {
        self.page_ordering_rules.iter().all(|(before, after)| {
            let Some(before_index) = pages.iter().rposition(|page| page == before) else {
                return true;
            };
            let Some(after_index) = pages.iter().position(|page| page == after) else {
                return true;
            };
            before_index < after_index
        })
    }

    pub fn correct_middle_pages(&self) -> u32 {
        self.pages_per_update
            .iter()
            .filter(|pages| self.pages_are_ordered(pages))
            .map(|pages| pages[pages.len() / 2])
            .sum()
    }

    pub fn incorrect_middle_pages(&self) -> u32 {
        self.pages_per_update
            .iter()
            .filter(|pages| !self.pages_are_ordered(pages))
            .map(|pages| {
                let mut pages = pages.clone();
                pages.sort_by(|a, b| self.compare_pages(*a, *b));
                pages[pages.len() / 2]
            })
            .sum()
    }

    fn compare_pages(&self, a: u32, b: u32) -> Ordering {
        if self
            .page_ordering_rules
            .iter()
            .any(|&(before, after)| a == after && b == before)
        {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl FromStr for SleighLaunchSafetyManual {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (ordering, pages) = s.split_once("\n\n").unwrap();
        let page_ordering_rules = ordering
            .lines()
            .map(|line| {
                let (before, after) = line.split_once('|').unwrap();
                (before.parse().unwrap(), after.parse().unwrap())
            })
            .collect();
        let pages_per_update = pages
            .lines()
            .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
            .collect();
        Ok(Self {
            page_ordering_rules,
            pages_per_update,
        })
    }
}
