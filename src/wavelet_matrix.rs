use super::counted_bitvec::CountedBitVec;

type WVT = u16;

#[derive(Debug, Clone, PartialEq)]
struct WaveletMatrix {
    bit_len: u32,
    size: u32,
    src: Vec<WVT>,
    bitvecs: Vec<CountedBitVec>,
}

impl Default for WaveletMatrix {
    fn default() -> Self {
        WaveletMatrix::new(0)
    }
}

impl WaveletMatrix {
    pub fn new(n: u32) -> Self {
        let size = ((n + u32::BITS - 1) / u32::BITS) * u32::BITS;
        let src = vec![0; size as usize];
        Self {
            size,
            src,
            bit_len: 0,
            bitvecs: vec![],
        }
    }

    fn set_preconstruct(&mut self, i: u32, val: WVT) {
        self.src[i as usize] = val;
    }

    fn construct(&mut self, bit_len: u32) {
        self.bit_len = bit_len;
        self.bitvecs = vec![CountedBitVec::new(self.size); bit_len as usize];

        let mut first = vec![];
        first.reserve(self.size as usize);
        let mut last = vec![];
        last.reserve(self.size as usize);

        for i in (0..bit_len).rev() {
            first.clear();
            last.clear();
            assert!(last.is_empty());
            for j in 0..self.size {
                let v = self.src[j as usize];
                if (v >> i) & 1 == 0 {
                    self.bitvecs[i as usize].set(j);
                    first.push(v);
                } else {
                    last.push(v);
                }
            }

            self.bitvecs[i as usize].update_counts();

            self.src.clear();
            self.src.append(&mut first);
            self.src.append(&mut last);
        }
    }

    // Number of values in interval less than val
    fn less_freq(&self, mut l: u32, mut r: u32, val: u32) -> u32 {
        let size = self.size;
        assert!((0..size + 1).contains(&l), "! 0 < {r} < {}", self.size);
        assert!((0..size + 1).contains(&r), "! 0 < {r} < {}", self.size);
        assert!((0..(1 << self.bit_len)).contains(&val));

        let mut res = 0;
        for i in (0..self.bit_len).rev() {
            let bvi = &self.bitvecs[i as usize];
            if (val >> i) & 1 == 0 {
                l = bvi.rank0(l);
                r = bvi.rank0(r);
            } else {
                res += bvi.rank0(r) - bvi.rank0(l);
                let bvi_r0_size = bvi.rank0(size);
                l = l - bvi.rank0(l) + bvi_r0_size;
                r = r - bvi.rank0(r) + bvi_r0_size;
            }
        }

        res
    }
    fn range_freq(&self, x0: u32, x1: u32, y0: u32, y1: u32) -> u32 {
        assert!(y1 > y0);
        self.less_freq(x0, x1, y1) - self.less_freq(x0, x1, y0)
    }
}

pub struct MedianWavelet {
    wms: [WaveletMatrix; 8],
    h: u32,
    w: u32,
}

impl MedianWavelet {
    const VAL_BIT_LEN: usize = 8;
    pub fn new(src: &[u8], h: u32, w: u32) -> Self {
        let hw = h * w;

        let w_bit_len = get_bit_len(w);

        let mut buf = vec![];
        for y in 0..h as usize {
            for x in 0..w {
                buf.push((src[y * w as usize + x as usize], x));
            }
        }

        let mut first = vec![];
        let mut last = vec![];
        let mut wms: [_; 8] = std::array::from_fn(|_| WaveletMatrix::default());
        for i in (0..Self::VAL_BIT_LEN).rev() {
            first.clear();
            last.clear();
            let mut wm = WaveletMatrix::new(hw);
            for j in 0..hw {
                let (v, x) = buf[j as usize];
                let (val, d) = if (v >> i) & 1 == 0 {
                    (x, &mut first)
                } else {
                    (w, &mut last)
                };
                wm.set_preconstruct(j, val as u16);
                d.push((v, x));
            }

            std::mem::swap(&mut buf, &mut first);
            buf.append(&mut last);
            wm.construct(w_bit_len);
            wms[i] = wm;
        }

        Self { w, h, wms }
    }

    pub fn quantile_2d(&self, x0: u32, x1: u32, y0: u32, y1: u32, mut k: u32) -> u8 {
        let w = self.w;
        let h = self.h;
        assert!((0..w + 1).contains(&x0));
        assert!((0..w + 1).contains(&x1));

        assert!((0..h + 1).contains(&y0));
        assert!((0..h + 1).contains(&y1));

        let mut l = y0 * w;
        let mut r = y1 * w;

        let mut res = 0;
        for i in (0..Self::VAL_BIT_LEN).rev() {
            let wm = &self.wms[i];
            let l_num = wm.range_freq(0, l, 0, w);
            let r_num = wm.range_freq(0, r, 0, w);
            let num = wm.range_freq(l, r, x0, x1);

            if k < num {
                l = l_num;
                r = r_num;
            } else {
                k -= num;
                let zeros = wm.range_freq(0, h * w, 0, w);
                l = l - l_num + zeros;
                r = r - r_num + zeros;
                res |= 1 << i;
            }
        }
        res
    }

    pub fn median_with_cut_border(&self, rad: u32, dst: &mut [u8]) {
        let w = self.w;
        let h = self.h;
        assert_eq!(dst.len(), (w * h) as usize);
        let diam = rad * 2 + 1;
        let med_idx = diam * diam / 2;

        for y in 0..h + 1 - diam {
            for x in 0..w + 1 - diam {
                let res = self.quantile_2d(x, x + diam, y, y + diam, med_idx);
                dst[(y * w + x) as usize] = res;
            }
        }
    }
}

fn get_bit_len(v: u32) -> u32 {
    if v == 0 {
        return 0;
    }
    32 - v.leading_zeros()
}

#[test]
fn test_wavelet_matrix_new() {
    const IMG: &[u8] = include_bytes!("../data/median_filters_before_srgb_flowers.png");
    let img = image::load_from_memory(&IMG).unwrap().into_luma8();
    let w = img.width();
    let h = img.height();

    let mw = MedianWavelet::new(&img, w, h);
}
