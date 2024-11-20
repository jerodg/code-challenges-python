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

/// This module provides a solution to the problem of taking characters from a string
/// such that each character appears at least `k` times in the resulting substring.
impl Solution {
    /// Takes characters from the string `s` such that each character appears at least `k` times.
    ///
    /// # Arguments
    ///
    /// * `s` - A string from which characters are taken.
    /// * `k` - An integer representing the minimum number of times each character should appear.
    ///
    /// # Returns
    ///
    /// * An integer representing the minimum length of the substring that satisfies the condition.
    ///   Returns `-1` if it's not possible to satisfy the condition.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = Solution::take_characters("aabbcc".to_string(), 2);
    /// assert_eq!(result, 6);
    /// ```
    pub fn take_characters(s: String, k: i32) -> i32 {
        // Vector to count occurrences of 'a', 'b', and 'c'
        let mut count = vec![0; 3];

        // Count the occurrences of each character in the string
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        // If any character appears less than `k` times, return -1
        if count.iter().min().unwrap() < &k {
            return -1;
        }

        // Convert the string to bytes for easier manipulation
        let s_bytes = s.as_bytes();
        // Initialize the result with the maximum possible value
        let mut res = i32::MAX;
        // Left pointer for the sliding window
        let mut l = 0;

        // Iterate over the string with the right pointer
        for r in 0..s_bytes.len() {
            // Decrease the count of the current character
            count[(s_bytes[r] - b'a') as usize] -= 1;

            // Adjust the left pointer to maintain the condition
            while count.iter().min().unwrap() < &k {
                count[(s_bytes[l] - b'a') as usize] += 1;
                l += 1;
            }
            // Update the result with the minimum length of the valid substring
            res = res.min(s_bytes.len() as i32 - (r as i32 - l as i32 + 1));
        }
        res
    }
}
