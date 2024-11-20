// Copyright © 2010-2024 <a href="https://github.com/jerodg/">JerodG</a>
//
// This program is free software: you can redistribute it and/or modify it under the terms of the
// Server Side Public License (SSPL) as published by MongoDB, Inc., either version 1 of the License,
// or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the SSPL
// for more details.
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software. You should have received a copy of the SSPL along with this
// program. If not, see <a href="https://www.mongodb.com/licensing/server-side-public-license">SSPL</a>.

/// This struct provides a solution to calculate the perimeter of an island in a grid.
impl Solution {
    /// Calculates the perimeter of the island in the given grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - A 2D vector representing the grid where `1` represents land and `0` represents water.
    ///
    /// # Returns
    ///
    /// * An integer representing the perimeter of the island.
    ///
    /// # Examples
    ///
    /// ```
    /// let grid = vec![
    ///     vec![0, 1, 0, 0],
    ///     vec![1, 1, 1, 0],
    ///     vec![0, 1, 0, 0],
    ///     vec![1, 1, 0, 0]
    /// ];
    /// let result = Solution::island_perimeter(grid);
    /// assert_eq!(result, 16);
    /// ```
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        // Initialize the perimeter to 0
        let mut perimeter = 0_i32;

        // Iterate over each cell in the grid
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                // Check if the current cell is land
                if grid[row][col] == 1 {
                    // Initialize the number of neighboring land cells to 0
                    let mut neighbors = 0_i32;

                    // Check the cell above the current cell
                    if row > 0 {
                        neighbors += grid[row - 1][col];
                    }

                    // Check the cell below the current cell
                    if row < grid.len() - 1 {
                        neighbors += grid[row + 1][col];
                    }

                    // Check the cell to the left of the current cell
                    if col > 0 {
                        neighbors += grid[row][col - 1];
                    }

                    // Check the cell to the right of the current cell
                    if col < grid[0].len() - 1 {
                        neighbors += grid[row][col + 1];
                    }

                    // Each land cell contributes 4 to the perimeter minus the number of neighboring land cells
                    perimeter += 4 - neighbors;
                }
            }
        }
        perimeter
    }
}