fn path_finder(maze: &str) -> bool {
    let mut lab = maze
        .split('\n')
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    static DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut res: Vec<(usize, usize)> =
        Vec::with_capacity(maze.chars().filter(|&c| c == '.').count());
    res.push((0, 0));

    fn find_n_push(row: i32, col: i32, lab: &[Vec<char>], res: &mut Vec<(usize, usize)>) {
        DIR.iter().for_each(|&(y, x)| {
            let (row, col) = (row + y, col + x);
            if row >= 0
                && col >= 0
                && row < lab.len() as i32
                && col < lab.len() as i32
                && '.' == lab[row as usize][col as usize]
            {
                // println!("about to push to row {} and col {} to res {:?}",row,col,res);
                res.push((row as usize, col as usize));
            }
        });
    }

    while let Some((y, x)) = res.pop() {
        println!("just poped out y{} x{}", y, x);
        lab[y][x] = 'x';
        find_n_push(y as i32, x as i32, &lab, &mut res);
        if matches!(lab[lab.len() - 1][lab[0].len() - 1], 'x') {
            return true;
        }
    }
    false
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// /// A square
// enum Sq {
//     Wall,
//     Fresh,
//     Front,
//     Visited,
// }
// fn path_finder(maze: &str) -> bool {
//     let mut matrix: Vec<Vec<_>> = maze
//         .lines()
//         .map(|l| {
//             l.chars()
//                 .map(|c| match c {
//                     'W' => Sq::Wall,
//                     '.' => Sq::Fresh,
//                     _ => panic!("Invalid input: {}", c),
//                 })
//                 .collect()
//         })
//         .collect();
//     assert!(matrix.len() != 0, "maze cannot have height of 0");
//     assert!(matrix.iter().all(|r| r.len() == matrix.len()), "not a square maze");
//     let size = matrix.len();
//
//     matrix[0][0] = Sq::Front;
//     loop {
//         // if end point reached
//         if matches!(matrix[size - 1][size - 1], Sq::Front | Sq::Visited) {
//             break true;
//         }
//         // if all available options exhausted
//         if !matrix.iter().flatten().any(|sq| matches!(sq, Sq::Front)) {
//             break false;
//         }
//         // breadth-first search one step
//         // not a strict implementation of breadth-first search because in-place mutation
//         // but it's good enough for this purpose
//         for y in 0..size {
//             for x in 0..size {
//                 if !matches!(matrix[y][x], Sq::Front) {
//                     continue;
//                 }
//                 // self
//                 matrix[y][x] = Sq::Visited;
//                 // north
//                 if let Some(sq @ Sq::Fresh) = y.checked_sub(1).map(|y_n| &mut matrix[y_n][x]) {
//                     *sq = Sq::Front;
//                 }
//                 // east
//                 if let Some(sq @ Sq::Fresh) = matrix[y].get_mut(x + 1) {
//                     *sq = Sq::Front;
//                 }
//                 // south
//                 if let Some(sq @ Sq::Fresh) = matrix.get_mut(y + 1).map(|v| &mut v[x]) {
//                     *sq = Sq::Front;
//                 }
//                 // west
//                 if let Some(sq @ Sq::Fresh) = x.checked_sub(1).map(|x_w| &mut matrix[y][x_w]) {
//                     *sq = Sq::Front;
//                 }
//             }
//         }
//     }
// }

// fn path_finder(maze: &str) -> bool {
//     let mut lab = maze
//         .split('\n')
//         .map(|line| line.chars().collect())
//         .collect::<Vec<Vec<_>>>();
//
//     static DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
//
//     fn get_valid_siblings(row: i32, col: i32, lab: &[Vec<char>]) -> Vec<(i32, i32)> {
//         DIR.iter()
//             .filter(|&&(r, c)| {
//                 r + row >= 0
//                     && c + col >= 0
//                     && r + row < lab.len() as i32
//                     && c + col < lab.len() as i32
//             })
//             .filter(|&&(r, c)| {
//                 Some(&'.') == lab.get((row + r) as usize).unwrap().get((col + c) as usize)
//             })
//             .map(|&(r, c)| (row + r, col + c))
//             .collect()
//     }
//
//     fn walk(row: i32, col: i32, v: &mut Vec<Vec<char>>) -> bool {
//         if row as usize == v.len() - 1 && col as usize == v[0].len() - 1 {
//             true
//         } else {
//             v[row as usize][col as usize] = 'X';
//             let t = get_valid_siblings(row, col, v);
//             if !t.is_empty() {
//                 t.iter().any(|(r, c)| walk(*r, *c, v))
//             } else {
//                 false
//             }
//         }
//     }
//     walk(0, 0, &mut lab)
// }


#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn basic() {
        test_maze("\
            ..W.\n\
            W...\n\
            ..W.\n
            .WW.",
                  true,
        );

        test_maze("\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\
            ",
                  true,
        );

        test_maze("\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            .....W\n\
            ....W.\
            ",
                  false,
        );
    }

    fn test_maze(maze: &str, expect: bool) {
        let actual = path_finder(maze);

        assert!(
            actual == expect,
            "Test failed!\n\
             Got:      {}\n\
             Expected: {}\n\
             Maze was: \n\
             {}",
            actual,
            expect,
            maze
        );
    }
}