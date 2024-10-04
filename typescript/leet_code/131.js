/*
 *  Copyright ©2010-2024 <a href="https://github.com/jerodg/">JerodG</a>
 *
 *  This program is free software: you can redistribute it and/or modify it under the terms of the
 *  Server Side Public License (SSPL) as published by MongoDB, Inc., either version 1 of the License,
 *  or (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
 *  even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the SSPL
 *  for more details.
 *
 *  The above copyright notice and this permission notice shall be included in all copies or
 *  substantial portions of the Software. You should have received a copy of the SSPL along with this
 *  program. If not, see SSPL.
 */
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
/**
 * Package leet_code
 *
 * This file contains the implementation of the `partition` function.
 * The function generates all possible palindrome partitioning of a given string.
 */
/**
 * Function to generate all possible palindrome partitioning of a given string.
 *
 * @param {string} s - The string to be partitioned.
 * @returns {string[][]} - An array of all possible palindrome partitioning of the string.
 *
 * @example
 *
 * partition("aab")
 * // returns [["a","a","b"],["aa","b"]]
 */
function partition(s) {
    // Initialize the result array
    var res = [];
    /**
     * Function to check if a given string is a palindrome.
     *
     * @param {string | any[]} str - The string to be checked.
     * @returns {boolean} - Returns `true` if the string is a palindrome, `false` otherwise.
     */
    var validPalindrome = function (str) {
        var left = 0;
        var right = str.length - 1;
        // Check each character from both ends of the string
        while (left <= right) {
            if (str[left] !== str[right]) {
                return false;
            }
            left++;
            right--;
        }
        return true;
    };
    /**
     * Recursive helper function to generate all possible palindrome partitioning of a given string.
     *
     * @param {string} str - The string to be partitioned.
     * @param {string[]} arr - The current partitioning of the string.
     */
    var helper = function (str, arr) {
        // Base case: if the string is empty, add the current partitioning to the result array
        if (str === "") {
            res.push(__spreadArray([], arr, true));
            return;
        }
        // Recursive case: for each substring of the string, if it is a palindrome, add it to the current partitioning
        // and recurse on the rest of the string
        for (var i = 1; i <= str.length; i++) {
            if (validPalindrome(str.slice(0, i))) {
                arr.push(str.slice(0, i));
                helper(str.slice(i), arr);
                arr.pop();
            }
        }
    };
    // Call the helper function with the string and an empty array
    helper(s, []);
    // Return the result array
    return res;
}