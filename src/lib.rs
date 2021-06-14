
fn path_finder(maze: &str) -> bool {

    let mut lab = maze
        .split("\n")
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    static DIR: [(i32,i32);4] = [(0,1),(1,0),(0,-1),(-1,0)];

    fn get_valid_siblings (row: i32, col: i32, lab: &Vec<Vec<char>>) -> Vec<(i32,i32)> {
        DIR.iter()
            .filter(|&&(r,c)| r+row >= 0 && c+col >= 0 && r+row < lab.len() as i32 && c+col < lab.len() as i32)
            .filter(|&&(r,c)| Some(&'.') == lab.get((row + r) as usize).unwrap().get((col + c) as usize))
            .map(|&(r,c)|(row+r,col+c))
            .collect()
    }

    fn walk(row: i32, col:i32, v: &mut Vec<Vec<char>>) -> bool {
        if row as usize == v.len()-1 && col as usize == v[0].len()-1 {
            true
        } else {
            v[row as usize][col as usize] = 'X';
            let t = get_valid_siblings(row,col,v);
            if t.len() != 0 {
                t.iter().any(|(r,c)| walk(*r,*c,v))
            } else {
                false
            }
        }
    }
    walk(0,0,&mut lab)
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