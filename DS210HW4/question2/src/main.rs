const SIZE: usize = 24;
const ITERATIONS: usize = 24;

fn count_neighbors(board: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    let size = SIZE as isize;
    
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            
            let neighbor_x = match i as isize + x {
                n if n < 0 => (size - 1) as usize,
                n if n >= size => 0,
                n => n as usize,
            };
            
            let neighbor_y = match j as isize + y {
                n if n < 0 => (size - 1) as usize,
                n if n >= size => 0,
                n => n as usize,
            };
            
            count += board[neighbor_x][neighbor_y];
        }
    }
    count
}

fn calculate_liveness(current_state: i32, live_neighbors: i32) -> i32 {
    if live_neighbors == 3 || (live_neighbors == 2 && current_state == 1) {
        1
    } else {
        0
    }
}

fn next_generation(current: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut next = vec![vec![0; SIZE]; SIZE];
    for x in 0..SIZE {
        for y in 0..SIZE {
            let live_neighbors = count_neighbors(current, x, y);
            next[x][y] = calculate_liveness(current[x][y], live_neighbors);
        }
    }
    next
}

fn display_gameboard(gameboard: &Vec<Vec<i32>>) {
    for row in gameboard {
        for &cell in row {
            print!("{}", if cell == 1 { "x  " } else { "·  " });
        }
        println!();
    }
}

fn main() {
    let mut board = vec![vec![0; SIZE]; SIZE];
    let initial_points = vec![(0,1), (1,2), (2,0), (2,1), (2,2)];
    for &(x, y) in &initial_points {
        board[x][y] = 1;
    }

    for i in 0..ITERATIONS {
        println!("Iteration: {}", i + 1);
        display_gameboard(&board);
        board = next_generation(&board);
    }
}

#[test]
fn test_newgeneration() {
    let mut board = vec![vec![0; SIZE]; SIZE];
    board[0][1] = 1;
    board[1][2] = 1;
    board[2][0] = 1;
    board[2][1] = 1;
    board[2][2] = 1;
    let new_board = next_generation(&board);
    let mut expected_board = new_board.clone();
    expected_board[1][0] = 1;
    expected_board[0][1] = 0;
    expected_board[2][0] = 0;
    expected_board[3][1] = 1;
    
    assert_eq!(new_board, expected_board);
}
#[test]
fn test_blinker_oscillator() {
    let mut board = vec![vec![0; SIZE]; SIZE];

    board[1][2] = 1;
    board[1][3] = 1;
    board[1][4] = 1;

    let new_board = next_generation(&board);

    let mut expected_board = vec![vec![0; SIZE]; SIZE];
    expected_board[0][3] = 1;
    expected_board[1][3] = 1;
    expected_board[2][3] = 1;

    assert_eq!(new_board, expected_board);
}

#[test]
fn test_calculate_liveness() {
    // Test case 1: Cell stays alive with 2 neighbors
    assert_eq!(calculate_liveness(1, 2), 1);
    
    // Test case 2: Cell dies with 2 neighbors if it was dead
    assert_eq!(calculate_liveness(0, 2), 0);
    
    // Test case 3: Cell becomes alive with 3 neighbors regardless of previous state
    assert_eq!(calculate_liveness(0, 3), 1);
    assert_eq!(calculate_liveness(1, 3), 1);
    
    // Test case 4: Cell dies with too many neighbors
    assert_eq!(calculate_liveness(1, 4), 0);
    
    // Test case 5: Cell dies with too few neighbors
    assert_eq!(calculate_liveness(1, 1), 0);
    
    // Test case 6: Cell stays dead with no neighbors
    assert_eq!(calculate_liveness(0, 0), 0);
}