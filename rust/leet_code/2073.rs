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

/// This struct provides a solution to calculate the time required to buy tickets.
///
/// The `Solution` struct contains a method to determine the total time required for a person
/// at position `k` in the queue to buy their tickets, given the number of tickets each person
/// in the queue wants to buy.
impl Solution {
    /// Calculates the time required for the person at position `k` to buy their tickets.
    ///
    /// # Arguments
    ///
    /// * `tickets` - A vector of integers where each integer represents the number of tickets
    ///               each person in the queue wants to buy.
    /// * `k` - An integer representing the position (0-indexed) of the person in the queue.
    ///
    /// # Returns
    ///
    /// * An integer representing the total time required for the person at position `k` to buy
    ///   their tickets.
    ///
    /// # Examples
    ///
    /// ```
    /// let tickets = vec![2, 3, 2];
    /// let k = 2;
    /// let result = Solution::time_required_to_buy(tickets, k);
    /// assert_eq!(result, 6);
    /// ```
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        // Iterate over the tickets vector with indices and values, and accumulate the total time
        tickets.iter().enumerate().fold(0, |s, (i, &v)|
            // Calculate the minimum time required for each person in the queue
            s + std::cmp::min(tickets[k as usize] - i32::from(i as i32 > k), v)
        )
    }
}