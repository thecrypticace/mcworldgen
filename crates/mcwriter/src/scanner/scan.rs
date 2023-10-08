use itertools::Itertools;

use super::blocks::BlockDescriptor;

pub struct ScanResult {
    pub regions: usize,
    pub chunks: usize,
    pub sections: usize,
    pub blocks: usize,
    pub found: Vec<BlockDescriptor>,
}

impl ScanResult {
    pub fn new() -> Self {
        ScanResult {
            regions: 0,
            chunks: 0,
            sections: 0,
            blocks: 0,
            found: vec![],
        }
    }

    pub fn combine(list: impl Iterator<Item = Self>) -> Self {
        let mut summary = ScanResult::new();

        for result in list {
            summary.regions += result.regions;
            summary.chunks += result.chunks;
            summary.sections += result.sections;
            summary.blocks += result.blocks;

            let mut found = result.found.clone();
            summary.found.append(&mut found);
        }

        summary
    }

    #[inline(always)]
    pub fn filtering_blocks<P>(self, predicate: P) -> Self
    where
        P: FnMut(&BlockDescriptor) -> bool,
    {
        self.replacing_blocks(|blocks| blocks.into_iter().filter(predicate).collect_vec())
    }

    #[inline(always)]
    pub fn replacing_blocks<F, U>(self, mapper: F) -> Self
    where
        Self: Sized,
        U: IntoIterator<Item = BlockDescriptor>,
        F: FnOnce(Vec<BlockDescriptor>) -> U,
    {
        ScanResult {
            regions: self.regions,
            chunks: self.chunks,
            sections: self.sections,
            blocks: self.blocks,
            found: mapper(self.found).into_iter().collect_vec(),
        }
    }
}
