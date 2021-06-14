fn path_finder(maze: &str) -> bool {
    let mut lab = maze
        .split("\n")
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let is_walkable_to= |row: i32, col: i32| -> bool {
        if let Some(x) = lab.get(row as usize).unwrap().get(col as usize) {
            *x != 'W' && *x != 'X'
        } else {
            false
        }
    };
    fn look_for_paths(v: &mut Vec<Vec<char>>, r: i32, c:i32) -> bool {
        v[r as usize][c as usize] = 'X';
        [(0,1),(1,0),(0,-1)(-1,0)].iter()
            .filter(|&(x,y)| is_walkable_to(c+y,r+x))
            .for_each(|(x,y)| look_for_paths(v))
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn basic() {
        test_maze("\
            .W.\n\
            .W.\n\
            ...\
            ",
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