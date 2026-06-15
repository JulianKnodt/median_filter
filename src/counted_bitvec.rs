#[derive(Debug, Clone, PartialEq)]
pub struct CountedBitVec {
    data: Vec<u32>,
    counts: Vec<u32>,
    size: u32,
}

pub fn rank(x: u32, i: u32) -> u32 {
    debug_assert!(i < 32, "{i}");
    let mask = (1 << i) - 1;
    (x & mask).count_ones()
}

impl CountedBitVec {
    pub fn new(size: u32) -> Self {
        let elems = size / u32::BITS + 1;
        let data = vec![0; elems as usize];
        let counts = vec![0; elems as usize];
        Self { data, counts, size }
    }
    pub fn set(&mut self, i: u32) {
        debug_assert!(i < self.size);
        let chunk = i / u32::BITS;
        self.data[chunk as usize] |= 1 << (i % u32::BITS);
    }

    pub fn update_counts(&mut self) {
        let mut prev = 0;
        for (i, bits) in self.data.iter().enumerate() {
            self.counts[i] = prev;
            prev += bits.count_ones();
        }
    }

    pub fn rank0(&self, i: u32) -> u32 {
        //assert!(i < self.size, "!{i} < {}", self.size);
        let c_i = (i / u32::BITS) as usize;
        let c = self.data[c_i];
        self.counts[c_i] + rank(c, i % u32::BITS)
    }
    /*
    pub fn rank1(&self, i: u32) -> u32 {
        i - self.rank0(i)
    }
    */
}

#[test]
fn test_counted_bitvec() {
    let mut cbv = CountedBitVec::new(129);
    assert_eq!(cbv.data.len(), 5);
    cbv.set(32);
    cbv.update_counts();
    assert_eq!(cbv.counts[4], 1);

    assert_eq!(cbv.rank0(50), 1);
    assert_eq!(cbv.rank0(31), 0);
    assert_eq!(cbv.rank0(32), 0);
    assert_eq!(cbv.rank0(33), 1);

    const N: u32 = 35;
    let mut cbv = CountedBitVec::new(129);
    cbv.set(N);
    cbv.update_counts();
    assert_eq!(cbv.counts[4], 1);

    assert_eq!(cbv.rank0(N), 0);
    assert_eq!(cbv.rank0(N + 1), 1);
    assert_eq!(cbv.rank0(N - 1), 0);
}
