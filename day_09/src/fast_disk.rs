use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
pub struct DiskMap {
    files: Vec<File>,
    free_space: VecDeque<FreeSpace>,
}

#[derive(Debug)]
struct File {
    id: usize,
    position: usize,
    size: usize,
}

#[derive(Debug)]
struct FreeSpace {
    position: usize,
    size: usize,
}

impl DiskMap {
    pub fn compact(&mut self) {
        for file in self.files.iter_mut().rev() {
            if let Some(free_space) = self
                .free_space
                .iter_mut()
                .take_while(|free_space| free_space.position < file.position)
                .find(|free_space| free_space.size >= file.size)
            {
                file.position = free_space.position;
                free_space.position += file.size;
                free_space.size -= file.size;
                if self.free_space[0].size == 0 {
                    self.free_space.pop_front();
                }
            }
        }
    }

    pub fn checksum(&self) -> usize {
        self.files
            .iter()
            .map(|file| file.id * (2 * file.position + file.size - 1) * file.size / 2)
            .sum()
    }
}

impl FromStr for DiskMap {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut files = Vec::new();
        let mut free_space = VecDeque::new();
        let mut map = s
            .trim_end()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize);
        let mut position = 0;
        let mut id = 0;
        loop {
            let file_size = map.next().unwrap();
            files.push(File {
                id,
                position,
                size: file_size,
            });
            position += file_size;
            id += 1;
            if let Some(free_space_size) = map.next() {
                free_space.push_back(FreeSpace {
                    position,
                    size: free_space_size,
                });
                position += free_space_size;
            } else {
                break;
            };
        }
        Ok(Self { files, free_space })
    }
}
