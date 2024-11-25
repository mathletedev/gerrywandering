# Gerrywandering

> An intuitive explanation of gerrymandering through a mathematical model.

⭐ Winner of the 2024 Congressional App Challege!

Video analysis: https://www.youtube.com/watch?v=B1UjuttHhng

## Gerrymandering: Binary Merge Algorithm

1. Construct a binary tree by recursively splitting the map into halves.
2. For each node, count the number of votes for each party.
3. For each parent node, starting from the leaves:
    1. If both child nodes are majority opposition votes, merge them
    2. Otherwise, keep the nodes separate
    3. This will always benefit the favoured party, as opposition votes are
        collected into fewer districts

## Boids: Voter Behaviour

- Alignment: Steer towards the average heading of the group
- Cohesion: Steer towards the average position of the group
- Separation: Steer away from local entities

## Works Cited

Reynolds, Craig. “Flocks, Herds, and Schools: A Distributed Behavioral Model.”
    *University of Toronto*, Computer Graphics, July 1987,
    www.cs.toronto.edu/~dt/siggraph97-course/cwr87/. 

"Coding Adventure: Boids." *YouTube*, uploaded by Sebastian Lague, 26 Aug. 2019,
    www.youtube.com/watch?v=bqtqltqcQhw

"Coding Challenge 124: Flocking Simulation." *YouTube*, uploaded by The Coding
    Train, 11 Dec. 2018, www.youtube.com/watch?v=mhjuuHl6qHM
