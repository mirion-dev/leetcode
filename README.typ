#let problem-data = json("resources/problems_all.json")
#let problem-list(..id) = list(..id.pos().map(id => link("tests/p" + str(id) + ".rs", problem-data.at(str(id)).title)))

#title[leetcode]

My LeetCode solutions for studying Rust. They usually have the optimum complexity in practice.

== Quests

- Data Structures and Algorithms
    - Array I #problem-list(1929, 1470, 485)
    - Array II #problem-list(645, 1365, 448)
    - Stack #problem-list(1441, 150, 636)
    - Monotonic Stack #problem-list(1475, 739, 84)
    - #strike[Monotonic Stack II] (Premium only)

== Plans

- Top Interview 150
    - _Array / String_ #problem-list(88, 27, 26, 80, 169, 189, 121, 122, 169, 189, 121, 122, 55, 45, 274, 380, 238, 134, 13, 12, 58, 14, 151, 6, 28)
    - _Two Pointers_ #problem-list(11, 15)
    - _Sliding Window_ #problem-list(3)
    - _Matrix_
    - _Hashmap_ #problem-list(1)
    - _Intervals_
    - _Stack_ #problem-list(150, 224)
    - _Linked List_ #problem-list(2, 19)
    - _Binary Tree General_
    - _Binary Tree BFS_
    - _Binary Search Tree_
    - _Graph General_
    - _Graph BFS_
    - _Trie_
    - _Backtracking_ #problem-list(17)
    - _Divide & Conquer_
    - _Kadane's Algorithm_ #problem-list(53)
    - _Binary Search_ #problem-list(4)
    - _Heap_ #problem-list(215)
    - _Bit Manipulation_
    - _Math_ #problem-list(9, 66, 172, 69, 50)
    - _1D DP_ #problem-list(70)
    - _Multidimensional DP_ #problem-list(5)

== Misc

#problem-list(7, 8, 10, 16, 18, 44, 227, 367)
