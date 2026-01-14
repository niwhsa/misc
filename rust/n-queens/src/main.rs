/// Check if a queen can be placed at (row, col) without attacking existing queens.
/// Time: O(N) - checks column and both diagonals up to current row
/// Space: O(1)
fn can_place(board: &[Vec<bool>], row: usize, col: usize) -> bool {
    let n = board.len();

    for i in 0..row {
        // Same column
        if board[i][col] { return false; }

        let offset = row - i;

        // Upper-left diagonal
        if col >= offset && board[i][col - offset] { return false; }

        // Upper-right diagonal
        if col + offset < n && board[i][col + offset] { return false; }
    }
    true
}

/// Backtracking solver: try placing a queen in each column of current row.
/// Time: O(N!) - worst case explores N choices for row 0, N-1 for row 1, etc.
///       Pruning via can_place significantly reduces actual branches explored.
/// Space: O(N) - recursion depth is N (one call per row)
fn solve_nqueens(board: &mut Vec<Vec<bool>>, row: usize) -> bool {
    let n = board.len();
    if row == n { return true; }

    for col in 0..n {
        if can_place(board, row, col) {
            board[row][col] = true;
            if solve_nqueens(board, row + 1) { return true; }
            board[row][col] = false;  // Backtrack
        }
    }
    false
}

/// Solve N-Queens: place N queens on NxN board so none attack each other.
/// Returns queen positions as column indices for each row, or None if no solution.
///
/// Time: O(N!) - backtracking with pruning
/// Space: O(N²) - board storage + O(N) recursion stack
fn n_queens(n: usize) -> Option<Vec<usize>> {
    let mut board = vec![vec![false; n]; n];
    
    if solve_nqueens(&mut board, 0) {
        // Extract queen positions (column index for each row)
        let positions: Vec<usize> = board
            .iter()
            .map(|row| row.iter().position(|&q| q).unwrap())
            .collect();
        Some(positions)
    } else {
        None
    }
}

fn main() {
    let solution = n_queens(8);
    
    assert!(solution.is_some(), "8-queens should have a solution");
    let positions = solution.unwrap();
    
    println!("8-Queens solution: {:?}", positions);
    
    // Print the board
    for row in 0..8 {
        for col in 0..8 {
            if positions[row] == col {
                print!("Q ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify solution using can_place: rebuild board row by row
    fn verify_solution(positions: &[usize]) -> bool {
        let n = positions.len();
        let mut board = vec![vec![false; n]; n];

        for (row, &col) in positions.iter().enumerate() {
            if !can_place(&board, row, col) { return false; }
            board[row][col] = true;
        }
        true
    }

    #[test]
    fn test_n_queens_1() {
        let solution = n_queens(1);
        assert_eq!(solution, Some(vec![0]));
    }

    #[test]
    fn test_n_queens_2_no_solution() {
        assert_eq!(n_queens(2), None);
    }

    #[test]
    fn test_n_queens_3_no_solution() {
        assert_eq!(n_queens(3), None);
    }

    #[test]
    fn test_n_queens_4() {
        let solution = n_queens(4).expect("4-queens has solutions");
        assert_eq!(solution.len(), 4);
        assert!(verify_solution(&solution));
    }

    #[test]
    fn test_n_queens_8() {
        let solution = n_queens(8).expect("8-queens has solutions");
        assert_eq!(solution.len(), 8);
        assert!(verify_solution(&solution));
    }

    #[test]
    fn test_verify_valid_solution() {
        // Known valid 4-queens solution
        assert!(verify_solution(&[1, 3, 0, 2]));
    }

    #[test]
    fn test_verify_invalid_same_column() {
        assert!(!verify_solution(&[0, 0, 2, 3]));
    }

    #[test]
    fn test_verify_invalid_diagonal() {
        assert!(!verify_solution(&[0, 1, 2, 3]));
    }

    #[test]
    fn test_can_place() {
        let mut board = vec![vec![false; 4]; 4];

        // Empty board, can place anywhere
        assert!(can_place(&board, 0, 0));
        assert!(can_place(&board, 0, 3));

        // Place queen at (0, 1)
        board[0][1] = true;

        // Can't place in same column
        assert!(!can_place(&board, 1, 1));
        assert!(!can_place(&board, 2, 1));

        // Can't place on diagonals
        assert!(!can_place(&board, 1, 0));  // upper-left
        assert!(!can_place(&board, 1, 2));  // upper-right

        // Can place at safe positions
        assert!(can_place(&board, 1, 3));
    }
}
