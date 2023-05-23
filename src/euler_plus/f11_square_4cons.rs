use std::io::{self, BufRead};

// test until n-4
// 00 2020
// 01 1920
// 02 1820
// 10 2019
// 20 2018
// 30 2017
fn f(grid:&Vec<Vec<i32>>)-> i32 {
    let mut max:i32 = 0;
    let mut grid_mult: i32;
    for i in 0..20{
        for j in 0..17{
            // vertical
            grid_mult = grid[j][i] * grid[j+1][i] * grid[j+2][i] * grid[j+3][i];
            if grid_mult > max{
                max = grid_mult;
            }
            
            // horizontal
            grid_mult = grid[i][j] * grid[i][j+1] * grid[i][j+2] * grid[i][j+3];
            if grid_mult > max{
                max = grid_mult;
            }
            
            // i = 1
            // j = 0
            if i+j < 20 && i+j+1 < 20 && i+j+2 < 20 && i+j+3 < 20{
                grid_mult = grid[i+j][j] * grid[i+j+1][j+1] * grid[i+j+2][j+2] * grid[i+j+3][j+3];
                if grid_mult > max{
                    max = grid_mult;
                }
                grid_mult = grid[j][i+j] * grid[j+1][i+j+1] * grid[j+2][i+j+2] * grid[j+3][i+j+3];
                if grid_mult > max{
                    max = grid_mult;
                }
            }
            // i = 4
            // j = 0
            if j <= i && j+1 <= i && j+2 <= i && j+3 <= i{
                grid_mult = grid[i-j][j] * grid[i-j-1][j+1] * grid[i-j-2][j+2] * grid[i-j-3][j+3];
                if grid_mult > max{
                    max = grid_mult;
                }
                grid_mult = grid[j][i-j] * grid[j+1][i-j-1] * grid[j+2][i-j-2] * grid[j+3][i-j-3];
                if grid_mult > max{
                    max = grid_mult;
                }
            }
        } 
    }
    max
}

pub fn f11() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(20_usize);

    for i in 0..20_usize {
        grid.push(Vec::with_capacity(20_usize));

        grid[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }
    
    println!("{}",f(&grid))
}
