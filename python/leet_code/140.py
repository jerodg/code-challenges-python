"""Copyright ©2010-2024 <a href="https://github.com/jerodg/">JerodG</a>.

This program is free software: you can redistribute it and/or modify it under the terms of the
Server Side Public License (SSPL) as published by MongoDB, Inc., either version 1 of the License,
or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the SSPL
for more details.

The above copyright notice and this permission notice shall be included in all copies or
substantial portions of the Software. You should have received a copy of the SSPL along with this
program. If not, see SSPL.
"""
import copy


class TrieNode:
    """A node in the Trie data structure used to store words."""

    def __init__(self):
        """Initializes a TrieNode with an empty dictionary for children and a boolean to mark the end of a word."""
        self.children = {}
        self.word = False

    def add_word(self, word: str) -> None:
        """
        Adds a word to the Trie.

        Args:
            word (str): The word to be added to the Trie.
        """
        cur = self
        for c in word:
            if c not in cur.children:
                cur.children[c] = TrieNode()
            cur = cur.children[c]
        cur.word = True


class Solution:
    """A class to solve the problem of word segmentation using a Trie and depth-first search."""

    def wordBreak(self, s: str, word_dict: list[str]) -> list[str]:
        """
        Finds all possible sentences that can be formed by segmenting the string `s` using words from `word_dict`.

        Args:
            s (str): The input string to be segmented.
            word_dict (list[str]): A list of valid words for segmentation.

        Returns:
            list[str]: A list of all possible sentences formed by valid segmentations.
        """
        root = TrieNode()
        for w in word_dict:
            root.add_word(w)

        res = []

        def dfs(i: int, node: TrieNode, word: str, a: list[str], flag: bool = False) -> None:
            """
            Performs depth-first search to find all valid segmentations.

            Args:
                i (int): The current index in the string `s`.
                node (TrieNode): The current node in the Trie.
                word (str): The current word being formed.
                a (list[str]): The current list of words formed so far.
                flag (bool): A flag indicating if a valid word has been formed.
            """
            if i == len(s):
                if flag:
                    res.append(" ".join(a))
                return

            if s[i] not in node.children:
                return

            word += s[i]
            node = node.children[s[i]]

            if node.word:
                a1 = copy.copy(a)
                a1.append(word)
                dfs(i + 1, root, '', a1, flag=True)

            dfs(i + 1, node, word, a)

        dfs(0, root, '', [])

        return res