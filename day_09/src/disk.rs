use std::str::FromStr;

pub struct DiskMap {
    disk: Vec<Block>,
}

enum Block {
    File { id: u32 },
    FreeSpace,
}

impl DiskMap {
    pub fn compact(&mut self) {
        let mut free_index = 0;
        loop {
            loop {
                match self.disk.get(free_index) {
                    Some(Block::File { .. }) => free_index += 1,
                    Some(Block::FreeSpace) => break,
                    None => return,
                }
            }
            self.disk[free_index] = loop {
                match self.disk.pop() {
                    Some(file @ Block::File { .. }) => break file,
                    Some(Block::FreeSpace) => {
                        if self.disk.len() == free_index + 1 {
                            return;
                        }
                    }
                    None => unreachable!(),
                }
            };
        }
    }

    pub fn checksum(&self) -> u32 {
        self.disk
            .iter()
            .enumerate()
            .map(|(position, block)| match block {
                Block::File { id } => u32::try_from(position).unwrap() * id,
                Block::FreeSpace => 0,
            })
            .sum()
    }
}

impl FromStr for DiskMap {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut disk = Vec::new();
        let mut map = s.chars().map(|c| c.to_digit(10).unwrap());
        for id in 0.. {
            if let Some(file_size) = map.next() {
                for _ in 0..file_size {
                    disk.push(Block::File { id });
                }
            } else {
                break;
            }
            if let Some(free_space) = map.next() {
                for _ in 0..free_space {
                    disk.push(Block::FreeSpace);
                }
            } else {
                break;
            }
        }
        Ok(Self { disk })
    }
}
