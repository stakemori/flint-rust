use bindings;
use libc::{c_ulong, c_long};

pub fn jacobi(x: c_long, y: c_ulong) -> i32 {
    unsafe { bindings::n_jacobi(x, y) as i32 }
}

pub fn remove(n: &mut c_ulong, p: c_ulong) -> u64 {
    unsafe { bindings::n_remove(n as *mut c_ulong, p) as u64 }
}

/// Return the hilbert symbol of `(u_a * p^a_expt, u_b * p^b_expt)`.
pub fn hilbert_symbol_odd_with_expt(
    a_expt: u64,
    b_expt: u64,
    u_a: c_long,
    u_b: c_long,
    p: c_ulong,
) -> i32 {
    match (is_even!(a_expt), is_even!(b_expt)) {
        (true, true) => 1,
        (true, false) => jacobi(u_a, p),
        (false, true) => jacobi(u_b, p),
        (false, false) => {
            // p equiv 1 mod 4
            if p & 0b11 == 0b01 {
                jacobi(u_a, p) * jacobi(u_b, p)
            } else {
                -jacobi(u_a, p) * jacobi(u_b, p)
            }
        }
    }
}

/// Return the hilbert symbol of `(u_a * p^val_a, u_b * p^val_b)` for `p` = 2.
pub fn hilbert_symbol_2_with_expt(val_a: u64, val_b: u64, u_a: c_long, u_b: c_long) -> i32 {
    let mut res_expt = 0;
    let a_eps = _eps_expt(u_a);
    let b_eps = _eps_expt(u_b);
    let a_om = _omega_expt(u_a);
    let b_om = _omega_expt(u_b);
    if !is_even!(a_eps) && !is_even!(b_eps) {
        res_expt += 1;
    }
    match (is_even!(val_a), is_even!(val_b)) {
        (true, false) => {
            res_expt += a_om;
        }
        (false, true) => {
            res_expt += b_om;
        }
        (false, false) => {
            res_expt += a_om + b_om;
        }
        _ => (),
    }
    if is_even!(res_expt) { 1 } else { -1 }
}

fn _eps_expt(a: c_long) -> i32 {
    if a & 0b11 == 0b01 { 0 } else { 1 }
}

fn _omega_expt(a: c_long) -> i32 {
    if (a & 0b111 == 1) | (a & 0b111 == 7) {
        0
    } else {
        1
    }
}

#[allow(dead_code)]
fn hilbert_symbol_odd(a: c_ulong, b: c_ulong, p: c_ulong) -> i32 {
    let mut u_a = a;
    let a_expt = remove(&mut u_a, p);
    let mut u_b = b;
    let b_expt = remove(&mut u_b, p);
    hilbert_symbol_odd_with_expt(a_expt, b_expt, u_a as i64, u_b as i64, p)
}

#[allow(dead_code)]
fn hilbert_symbol_2(a: c_ulong, b: c_ulong) -> i32 {
    let mut u_a = a;
    let mut u_b = b;
    let a_expt = remove(&mut u_a, 2);
    let b_expt = remove(&mut u_b, 2);
    hilbert_symbol_2_with_expt(a_expt, b_expt, u_a as i64, u_b as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hilbert_symbol_odd() {
        let p = 5;
        let a = 2;
        assert_eq!(hilbert_symbol_odd(a, a, p), 1);
        assert_eq!(hilbert_symbol_odd(a, p, p), -1);
        assert_eq!(hilbert_symbol_odd(p, a, p), -1);
        assert_eq!(hilbert_symbol_odd(p, p, p), 1);
        assert_eq!(hilbert_symbol_odd(p, (p * a), p), -1);

        let p = 7;
        let a = 3;
        assert_eq!(hilbert_symbol_odd(a, a, p), 1);
        assert_eq!(hilbert_symbol_odd(a, p, p), -1);
        assert_eq!(hilbert_symbol_odd(p, a, p), -1);
        assert_eq!(hilbert_symbol_odd(p, p, p), -1);
        assert_eq!(hilbert_symbol_odd(p, (p * a), p), 1);
    }

    #[test]
    fn test_hilbert_symbol2() {
        let mut res = Vec::new();
        let mut res1 = Vec::new();
        let mut res2 = Vec::new();
        for &i in &[1, 3, 5, 7] {
            for &j in &[1, 3, 5, 7] {
                let a = i;
                let b = j;
                let c = 2 * i;
                let d = 2 * j;
                assert_eq!(hilbert_symbol_2(a, b), hilbert_symbol_2(b, a));
                assert_eq!(hilbert_symbol_2(a, d), hilbert_symbol_2(d, a));
                if hilbert_symbol_2(a, d) == -1 {
                    res.push((i, j));
                }
                if hilbert_symbol_2(c, d) == -1 {
                    res1.push((i, j));
                }
                if hilbert_symbol_2(a, b) == -1 {
                    res2.push((i, j));
                }
            }
        }
        assert_eq!(
            res,
            vec![
                (3, 1),
                (3, 5),
                (5, 1),
                (5, 3),
                (5, 5),
                (5, 7),
                (7, 3),
                (7, 7),
            ]
        );
        assert_eq!(
            res1,
            vec![
                (1, 3),
                (1, 5),
                (3, 1),
                (3, 3),
                (5, 1),
                (5, 7),
                (7, 5),
                (7, 7),
            ]
        );
        assert_eq!(res2, vec![(3, 3), (3, 7), (7, 3), (7, 7)]);
    }
}
