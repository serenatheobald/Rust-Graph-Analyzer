# Rust-Graph-Analyzer

This Rust program is designed to analyze a graph dataset (specifically, from a file named "data.txt") in terms of its vertex connectivity and degree distribution. It performs the following major steps:

## Reading the Graph Data: 
It reads a list of edges from a file, where each line (except the first) represents an edge between two vertices in the graph. The first line indicates the total number of vertices, n, although this value is not used directly in the subsequent steps due to an alternative method of determining n based on the data read.

## Mapping Vertex IDs: 
It creates a new, compact mapping for vertex IDs. Original vertex IDs are mapped to new IDs ranging from 0 up to new_node_id - 1, ensuring a continuous range of IDs. This is useful for avoiding gaps in the vertex ID space, especially if the original IDs are sparse.

## Building the Graph: 
Using the remapped vertex IDs, it constructs the adjacency list representation of the graph. Each vertex's list contains the IDs of vertices to which it is directly connected.

## Calculating Degree Distribution: 
The program calculates the degree of each vertex (i.e., the number of edges connected to the vertex) and computes the degree distribution across all vertices. The degree distribution is normalized by the total number of vertices to represent the fraction of vertices that have each degree.

## Estimating the Best Beta for a Power-Law Distribution:
It attempts to fit the degree distribution of the graph to a power-law distribution, characterized by a parameter β (beta). The fitting process involves iterating over a range of possible β values, calculating the sum of absolute differences between the observed degree distribution and the power-law distribution for each β, and selecting the β that minimizes this total error. This process is an attempt to identify which power-law distribution best approximates the graph's degree distribution.

In summary, the code analyzes the structural properties of a graph, specifically focusing on its degree distribution and how well this distribution can be modeled by a power-law. This kind of analysis helps us understand the characteristics of networks, such as social networks, protein interaction networks, and others, where the degree distribution is often an important indicator of the network's structure and dynamics.
