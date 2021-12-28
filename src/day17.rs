use std::mem::swap;

use roots::{find_roots_quadratic, Roots};

fn simulate(v_0: f64, t: f64) -> f64 {
    -0.5f64 * (t * t) + ((v_0 + 0.5f64) * t)
}

fn solve_for_t(v_0: f64, y: f64) -> Option<f64> {
    let a = -0.5f64;
    let b = v_0 + 0.5f64;
    let c = -y;

    match find_roots_quadratic(a, b, c) {
        Roots::Two(roots) => Some(roots[1]),
        Roots::One(roots) => Some(roots[0]),
        _ => None,
    }
}

fn step_number(t: f64, is_first: bool) -> u64 {
    if !is_first {
        return t as u64;
    }

    if t.fract() > 1e-9 {
        t as u64
    } else {
        //Count being on the edge as the previous step
        (t - 1f64) as u64
    }
}

fn same_step(t0: f64, t1: f64) -> bool {
    let t0 = step_number(t0, true);
    let t1 = step_number(t1, false);
    return t0 == t1;
}

fn y_on_target(y0: f64, y_start: f64, y_end: f64) -> Option<(u64, u64)> {
    let t1 = solve_for_t(y0, y_start);
    let t0 = solve_for_t(y0, y_end);
    if t0.is_none() {
        return None;
    }

    let mut t0 = t0.unwrap().ceil();
    if let Some(t1) = t1 {
        let mut t1 = t1.floor();
        if !same_step(t0, t1) {
            if t0 > t1 {
                swap(&mut t0, &mut t1);
            }
            Some((t0 as u64, t1 as u64))
        } else {
            None
        }
    } else {
        None // t1 is never reached, but t0 is
    }
}

fn find_opt_y(y_start: i32, y_end: i32) -> u64 {
    let y_start = y_start as f64;
    let y_end = y_end as f64;
    (0..10000)
        .map(|i| i as f64)
        .filter(|y0| y_on_target(*y0, y_start, y_end).is_some())
        .map(|y| y as u64)
        .max()
        .unwrap()
}

fn peak(v_0: f64) -> f64 {
    simulate(v_0, v_0 + 0.5f64).round()
}

fn simulate_x(v_0: i32, (x_s, x_e): (i32, i32)) -> Option<(i32, (u64, u64))> {
    let mut v = v_0;
    let mut p = 0;
    let mut range: Option<(u64, u64)> = None;
    for t in 0..100000 {
        p += v;
        if v.abs() > 0 {
            v -= v.signum();
        }

        if p >= x_s && p <= x_e {
            range = match range {
                Some((s, _)) => Some((s, t + 1)),
                None => Some((t + 1, t + 1)),
            };
        }
        if v == 0 || p > x_e {
            return match range {
                None => None,
                Some(range) => {
                    if v == 0 {
                        Some((v_0, (range.0, u64::MAX)))
                    } else {
                        Some((v_0, range))
                    }
                }
            };
        }
    }
    return None;
}

fn all_solutions((x_s, x_e): (i32, i32), (y_s, y_e): (i32, i32)) -> Vec<(i32, i32)> {
    let all_ys = all_solutions_y(y_s, y_e);
    let all_xs = all_solutions_x(x_s, x_e);

    all_ys
        .iter()
        .map(|(y, (s_y, e_y))| {
            all_xs
                .iter()
                .filter_map(|(x, (s_x, e_x))| {
                    let overlaps = s_x <= e_y && s_y <= e_x;

                    if overlaps {
                        Some((*x, *y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn all_solutions_y(y_s: i32, y_e: i32) -> Vec<(i32, (u64, u64))> {
    let y_start = y_s as f64;
    let y_end = y_e as f64;

    (-1000..1000)
        .map(|i| i as f64)
        .filter_map(|y0| {
            if let Some((t0, t1)) = y_on_target(y0, y_start, y_end) {
                Some((y0 as i32, (t0, t1)))
            } else {
                None
            }
        })
        .collect()
}

fn all_solutions_x(x_s: i32, x_e: i32) -> Vec<(i32, (u64, u64))> {
    (-500..500)
        .filter_map(|x0| simulate_x(x0, (x_s, x_e)))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn func() {
        let v_0 = 3f64;
        assert_eq!(
            vec![0, 3, 5, 6, 6, 5, 3, 0, -4, -9, -15, -22, -30],
            (0..13)
                .map(|t| simulate(v_0, t as f64) as i32)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn solve() {
        assert_eq!(5, solve_for_t(3f64, 5f64).unwrap() as u64);
        assert_eq!(10, solve_for_t(9f64, 45f64).unwrap() as u64);
        assert_eq!(6, solve_for_t(2f64, -5f64).unwrap() as u64);
        assert_eq!(8, solve_for_t(3f64, -5f64).unwrap() as u64);
        assert_eq!(3, solve_for_t(0f64, -5f64).unwrap() as u64);
    }

    //
    #[test]
    fn part_one_small() {
        let opt_y = find_opt_y(-10, -5);
        assert_eq!(9, opt_y);
        assert_eq!(45, peak(opt_y as f64) as u64);
    }

    #[test]
    fn part_one() {
        let opt_y = find_opt_y(-126, -69);
        assert_eq!(125, opt_y);
        assert_eq!(7875, peak(opt_y as f64) as u64);
    }

    #[test]
    fn part_two_small() {
        let mut expected = test_output();

        expected.sort_by(|(a0, b0), (a1, b1)| a0.cmp(a1).then_with(|| b0.cmp(b1)));
        let mut all_solutions = all_solutions((20, 30), (-10, -5));
        all_solutions.sort_by(|(a0, b0), (a1, b1)| a0.cmp(a1).then_with(|| b0.cmp(b1)));

        assert_eq!(expected, all_solutions);
        assert_eq!(expected.len(), all_solutions.len());
    }

    #[test]
    fn part_two_xs() {
        assert!(simulate_x(6, (20, 30)).is_some());
        let mut expected = test_output().iter().map(|(x, _)| *x).collect::<Vec<_>>();
        expected.sort();
        expected.dedup();
        let all_xs = all_solutions_x(20, 30)
            .iter()
            .map(|(t, _)| *t)
            .collect::<Vec<_>>();

        assert_eq!(expected, all_xs);
    }

    #[test]
    fn pt_2_same_step() {
        assert_eq!(false, same_step(1.0, 1.884));
        assert_eq!(false, same_step(8.6, 9.17));
        assert_eq!(false, same_step(10.0, 10.844288770224761));
        assert_eq!(true, same_step(23.426860441876563, 23.844288770224761));
    }

    #[test]
    fn part_two_ys() {
        let mut expected = test_output().iter().map(|(_, y)| *y).collect::<Vec<_>>();
        expected.sort();
        expected.dedup();

        assert_eq!(
            expected,
            all_solutions_y(-10, -5)
                .iter()
                .map(|(y, _)| *y)
                .collect::<Vec<_>>()
        );
    }

    fn test_output() -> Vec<(i32, i32)> {
        vec![
            (23, -10),
            (25, -9),
            (27, -5),
            (29, -6),
            (22, -6),
            (21, -7),
            (9, 0),
            (27, -7),
            (24, -5),
            (25, -7),
            (26, -6),
            (25, -5),
            (6, 8),
            (11, -2),
            (20, -5),
            (29, -10),
            (6, 3),
            (28, -7),
            (8, 0),
            (30, -6),
            (29, -8),
            (20, -10),
            (6, 7),
            (6, 4),
            (6, 1),
            (14, -4),
            (21, -6),
            (26, -10),
            (7, -1),
            (7, 7),
            (8, -1),
            (21, -9),
            (6, 2),
            (20, -7),
            (30, -10),
            (14, -3),
            (20, -8),
            (13, -2),
            (7, 3),
            (28, -8),
            (29, -9),
            (15, -3),
            (22, -5),
            (26, -8),
            (25, -8),
            (25, -6),
            (15, -4),
            (9, -2),
            (15, -2),
            (12, -2),
            (28, -9),
            (12, -3),
            (24, -6),
            (23, -7),
            (25, -10),
            (7, 8),
            (11, -3),
            (26, -7),
            (7, 1),
            (23, -9),
            (6, 0),
            (22, -10),
            (27, -6),
            (8, 1),
            (22, -8),
            (13, -4),
            (7, 6),
            (28, -6),
            (11, -4),
            (12, -4),
            (26, -9),
            (7, 4),
            (24, -10),
            (23, -8),
            (30, -8),
            (7, 0),
            (9, -1),
            (10, -1),
            (26, -5),
            (22, -9),
            (6, 5),
            (7, 5),
            (23, -6),
            (28, -10),
            (10, -2),
            (11, -1),
            (20, -9),
            (14, -2),
            (29, -7),
            (13, -3),
            (23, -5),
            (24, -8),
            (27, -9),
            (30, -7),
            (28, -5),
            (21, -10),
            (7, 9),
            (6, 6),
            (21, -5),
            (27, -10),
            (7, 2),
            (30, -9),
            (21, -8),
            (22, -7),
            (24, -9),
            (20, -6),
            (6, 9),
            (29, -5),
            (8, -2),
            (27, -8),
            (30, -5),
            (24, -7),
        ]
    }

    #[test]
    fn part_two() {
        assert_eq!(2321, all_solutions((217, 240), (-126, -69)).len());
    }
}
