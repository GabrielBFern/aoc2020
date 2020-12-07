#[derive(PartialEq, Debug)]
enum MapTile {
    Floor,
    Tree,
}

fn parse_map(data: &str) -> Vec<Vec<MapTile>> {
    data.lines()
        .map(|l| {
            l.chars()
                .map(|e| {
                    if e == '#' {
                        MapTile::Tree
                    } else {
                        MapTile::Floor
                    }
                })
                .collect()
        })
        .collect()
}

fn travese_tree(map: &[Vec<MapTile>], (right, down): (usize, usize)) -> usize {
    let size = map[0].len();
    map.iter()
        .step_by(down)
        .enumerate()
        .fold(0, |tot, (pos, vec)| {
            if vec[(pos * right) % size] == MapTile::Tree {
                tot + 1
            } else {
                tot
            }
        })
}

fn part1(data: &str) {
    let map = parse_map(data);
    println!("Part 1: {:}", travese_tree(&map, (1, 2)));
}

fn part2(data: &str) {
    let map = parse_map(data);
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let tot: usize = slopes.iter().map(|e| travese_tree(&map, *e)).product();
    println!("Part 2: {:}", tot);
}

fn main() {
    let input = include_str!("../../input/d3");
    part1(input);
    part2(input);
}

#[cfg(test)]
mod test_tree {
    use super::*;

    #[test]
    fn test_travese() {
        let map = parse_map("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#");
        assert_eq!(2, travese_tree(&map, (1, 1)));
        assert_eq!(7, travese_tree(&map, (3, 1)));
        assert_eq!(3, travese_tree(&map, (5, 1)));
        assert_eq!(4, travese_tree(&map, (7, 1)));
        assert_eq!(2, travese_tree(&map, (1, 2)));
    }
}
