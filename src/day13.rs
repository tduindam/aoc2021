use std::cmp::Ordering;

use crate::reader::split_lines;

type Point = (i32, i32);

#[derive(Debug, Copy, Clone)]
enum Fold {
    X(i32),
    Y(i32),
}

fn sort_point((x1, y1): &Point, (x2, y2): &Point) -> Ordering {
    match y1.cmp(y2) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => x1.cmp(x2),
    }
}

fn print(mut points: Vec<Point>) {
    let (size_x, size_y) = grid_size(&points);
    points.sort_by(sort_point);
    let mut iter = points.iter();
    let mut cur_point = *iter.next().unwrap();
    for y in 0..size_y {
        for x in 0..size_x {
            let has_point = (x, y) == cur_point;
            print!("{}", if has_point { "#" } else { "." });
            if has_point {
                if let Some(p) = iter.next() {
                    //If we run out of iter we just keep the old point which will never match
                    cur_point = *p;
                }
            }
        }
        println!()
    }
}

fn grid_size(points: &Vec<Point>) -> (i32, i32) {
    let (max_x, _) = *points.iter().max_by_key(|(x, _)| x).unwrap();
    let (_, max_y) = *points.iter().max_by_key(|(_, y)| y).unwrap();
    (max_x + 1, max_y + 1)
}
//mirror 5:
// 0 1 2 3 4 5 6 7 8
// 0 1 2 3 4 5 4 3 2
// p - fold
// 6 - 1 - 1 4
// 6 - 2 - 1 3

fn fold_one(mirror: i32, p: i32) -> i32 {
    if p < mirror {
        p
    } else {
        mirror - (p - mirror)
    }
}

fn fold(points: Vec<Point>, fold: Fold) -> Vec<Point> {
    let mut points: Vec<Point> = points
        .into_iter()
        .map(|(x, y)| match fold {
            Fold::X(f) => (fold_one(f, x), y),
            Fold::Y(f) => (x, fold_one(f, y)),
        })
        .collect();

    points.sort_by(sort_point);
    points.dedup();
    points
}

fn parse_pos(input: &String) -> Option<Point> {
    let mut chunks = input.split(',');
    let x = chunks.next();
    let y = chunks.next();
    if x.is_none() || y.is_none() {
        return None;
    }

    let x = x.unwrap().parse::<i32>().ok();
    let y = y.unwrap().parse::<i32>().ok();
    if x.is_none() || y.is_none() {
        return None;
    }
    Some((x.unwrap(), y.unwrap()))
}

fn parse_fold(input: &String) -> Option<Fold> {
    let mut chunks = input.split('=');
    let dir = chunks.next();
    let length = chunks.next();
    if dir.is_none() || length.is_none() {
        return None;
    }

    let dir_is_x = dir.unwrap().contains('x');
    let length = length.unwrap().parse::<i32>().ok();
    if length.is_none() {
        return None;
    }
    match dir_is_x {
        true => Some(Fold::X(length.unwrap())),
        false => Some(Fold::Y(length.unwrap())),
    }
}

fn parse(lines: Vec<String>) -> (Vec<Point>, Vec<Fold>) {
    let positions: Vec<_> = lines.iter().filter_map(parse_pos).collect();
    let folds = lines.iter().filter_map(parse_fold).collect();
    (positions, folds)
}

#[cfg(test)]
mod test {
    use crate::reader::read_lines_filter_ok;

    use super::*;

    #[test]
    fn fold_one_test() {
        // 0 1 2 3 4 5 6 7 8
        // 0 1 2 3 4 5 4 3 2
        // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
        // 0 1 2 3 4 5 6 7 6 5 4  3  2  1  0
        assert_eq!(4, fold_one(5, 6));
        assert_eq!(4, fold_one(5, 4));
        assert_eq!(2, fold_one(5, 8));
        assert_eq!(2, fold_one(7, 12));
        assert_eq!(4, fold_one(7, 10));
        assert_eq!(0, fold_one(7, 14));
    }

    #[test]
    fn part_one_small() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let (pos, folds) = parse(split_lines(input));
        assert_eq!(18, pos.len());
        assert_eq!(2, folds.len());
        let folded = fold(pos, folds[0]);
        assert_eq!(17, folded.len());
    }

    #[test]
    fn part_one_real() {
        let (pos, folds) = parse(read_lines_filter_ok("input/day13"));
        let folded = fold(pos, folds[0]);
        assert_eq!(751, folded.len())
    }

    #[test]
    fn part_two_real() {
        let (mut pos, folds) = parse(read_lines_filter_ok("input/day13"));

        for f in folds {
            pos = fold(pos, f);
        }
        print(pos);
    }
}
