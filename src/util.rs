/// log2 of x floored
pub(crate) fn log2(x: u32) -> u32 {
    if x == 0 {
        return 0;
    }

    let mut k = 0;
    while (x >> k) > 0 {
        k += 1;
    }
    k - 1
}

/// The number of nodes needed to represent a tree with n leaves.
pub(crate) fn num_nodes(x: u32) -> u32 {
    if x == 0 {
        0
    } else {
        2 * (x - 1) + 1
    }
}

pub(crate) fn level(x: u32) -> u32 {
    // XXX: masked log2 really
    if x & 0x01 == 0 {
        return 0;
    }

    let mut k = 0;
    while ((x >> k) & 0x01) == 1 {
        k += 1;
    }
    k
}

pub(crate) fn left(x: u32) -> u32 {
    let k = level(x);
    if k == 0 {
        panic!("leaf nodes have no children");
    }

    x ^ (0x01 << (k - 1))
}

pub(crate) fn right(x: u32) -> u32 {
    let k = level(x);
    if k == 0 {
        panic!("leaf nodes have no children");
    }

    // This is different from RFC calculation.
    // It's only returning an ID, we don't care if there's a node or not.
    let r = x ^ (0x03 << (k - 1));
    r
}

pub(crate) fn sibling(x: u32) -> u32 {
    let p = parent(x);
    if x < p {
        right(p)
    } else {
        left(p)
    }
}

pub(crate) fn parent(x: u32) -> u32 {
    let k = level(x);
    // println!(" >>> x: {}\nk: {}", x, k+1);
    let b = (x >> (k + 1)) & 0x01;
    (x | (1 << k)) ^ (b << (k + 1))
}

pub(crate) fn direct_path(x: u32, r: u32) -> Vec<u32> {
    let mut d = Vec::new();
    if x == r {
        return d;
    }

    let mut n = x;
    while n != r {
        n = parent(n);
        d.push(n);
    }
    d
}
