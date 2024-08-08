#let codeblock(filename) = {
  let code = read(filename)
  let split_start = code.split("//---START---\n")
  let without_start = if split_start.len() > 1 { split_start.at(1) } else { split_start.at(0) }
  let without_end = without_start.split("//---END---").at(0)
  line(length: 100%)
  pad(left: 2em, right: 2em, 
    raw(without_end, lang: "cpp"))
  
  line(length: 100%)
}

== Data Structures
- SegTree (maybe also lazy)
- Union Find (maybe with rollback)
- Link Cut Tree (if necessary, don't really know it)

== Graphs
- TopoSort
- dijkstra
- BFS/DFS
- MinCut/MaxFlow
- finding cycles
- MST
- Strongly connected components

== Algorithms
=== Prime Sieve:
Runs in $cal(O)(n)$ and `lp[i]` stores the lowest prime divisor of $i$. Can be used as a fast prime sieve and for factorizing lots of numbers.

#codeblock("algorithms/linear_prime_sieve.cpp")
- fast exponentiation
- z-function
Runs in $cal(O)n$ and `z[i]` stores how long of a prefix match the string has with the substring starting at $i$. Can be used for lots of prefix, suffix and string matching.

#codeblock("algorithms/z_function.cpp")

== C++ Details
- important iterator functions:
  - sort
  - reverse
  - ...
- data structures and their functions:
   - set, unordered set
   - vector
   - queue, priority queue, deque
   - pair, custom structs
- hashes, custom hash functions
- min, max, pow, ... stl functions
- lambdas syntax + usecases

== Math
- probability distributions, including mean and std deviation
- matrix inverse with good numerical stability
- some series limits (harmonic series, ...)
- dp optimizations
- trigonometric identities (sin, cos, tan, ...)
- gcd, lcm
- Chinese remainder theorem
- mod arithmetic + inverse

== Geometry
- area of polygon
- line, point and plane intersections
- convex hull
- angle between vectors
