pub fn sort_3<T>(d: &mut [T; 3])
where
    T: Ord,
{
    if d[0] > d[2] {
        d.swap(0, 2);
    }
    if d[0] > d[1] {
        d.swap(0, 1);
    }
    if d[1] > d[2] {
        d.swap(1, 2);
    }
}

macro_rules! sorting_layer {
    ($arr: ident, $layer: expr) => {{
        for (i, j) in $layer {
            if $arr[i] > $arr[j] {
                $arr.swap(i, j);
            }
        }
    }};
}

pub fn sort_5<T>(d: &mut [T; 5])
where
    T: Ord,
{
    sorting_layer!(d, [(0, 3), (1, 4)]);
    sorting_layer!(d, [(0, 2), (1, 3)]);
    sorting_layer!(d, [(0, 1), (2, 4)]);
    sorting_layer!(d, [(1, 2), (3, 4)]);
    sorting_layer!(d, [(2, 3)]);
}

pub fn sort_7<T>(d: &mut [T; 7])
where
    T: Ord,
{
    sorting_layer!(d, [(0, 6), (2, 3), (4, 5)]);
    sorting_layer!(d, [(0, 2), (1, 4), (3, 6)]);
    sorting_layer!(d, [(0, 1), (2, 5), (3, 4)]);
    sorting_layer!(d, [(1, 2), (4, 6)]);
    sorting_layer!(d, [(2, 3), (4, 5)]);
    sorting_layer!(d, [(1, 2), (3, 4), (5, 6)]);
}
