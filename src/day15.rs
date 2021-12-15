use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::neighbors::{neighbors_straight, PosType};
use crate::reader::{parse_grid_lines, read_lines_filter_ok};

pub fn main() {
    println!(
        "Day 15-1 {}",
        find_path_lines(read_lines_filter_ok("input/day15")).unwrap()
    );
    let (size, grid) = expand(read_lines_filter_ok("input/day15"));
    println!("Day 15-2 {}", find_path(size, grid).unwrap());
}

fn find_path_lines(lines: Vec<String>) -> Option<usize> {
    let (size, grid) = parse_grid_lines(lines);
    find_path(size, grid)
}

fn find_path((row_size, col_size): (usize, usize), grid: Vec<u32>) -> Option<usize> {
    let ((row_size, col_size), grid) = parse_grid_lines(lines);
    let adj_list: Vec<_> = (0..grid.len())
        .map(|i| PosType::from_index(i as u32, (row_size as u32, col_size as u32)))
        .map(|(p, t)| {
            neighbors_straight(p, t)
                .map(|(x, y)| (x as usize + y as usize * row_size) as usize)
                .collect::<Vec<_>>()
        })
        .collect();
    let goal = grid.len() - 1;
    path(adj_list, grid, goal)
}

fn enlarge(grid: Vec<u32>) -> Vec<u32> {
    grid
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn path(adj_list: Vec<Vec<usize>>, cost_map: Vec<u32>, end: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();
    heap.push(State {
        position: 0,
        cost: 0,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }
        for index in &adj_list[position] {
            let next = State {
                cost: cost + cost_map[*index] as usize,
                position: *index,
            };
            if next.cost < dist[*index] {
                heap.push(next);
                dist[*index] = next.cost;
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::reader::split_lines;

    use super::*;

    #[test]
    fn part_one_small() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        let lines = split_lines(input);

        assert_eq!(Some(40), find_path_lines(lines));
    }

    #[test]
    fn part_one() {
        assert_eq!(
            Some(720),
            find_path_lines(read_lines_filter_ok("input/day15"))
        );
    }

    #[test]
    fn part_two_small() {
        let input = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";

        let lines = split_lines(input);

        assert_eq!(Some(315), find_path_lines(lines));
    }

    #[test]
    fn part_two() {
        let (size, grid) = expand(read_lines_filter_ok("input/day15"));
        assert_eq!(Some(720), find_path(size, grid));
    }
}
