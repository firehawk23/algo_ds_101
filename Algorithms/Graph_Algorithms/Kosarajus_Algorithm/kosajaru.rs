
use std::vec;
type Node  = usize;           
type Graph = Vec<Vec<Node>>; // A directed graph, in adjacency list
type Stack = Vec<Node>;      // The rust std does not provides a default stack, but we can use a vector instead


// Example
fn main() {
    let graph = vec![
        vec![2,3],
        vec![0],
        vec![1],
        vec![4],
        vec![],
    ];
    let n_nodes = 5;

    println!("Provided graph: ");
    println!("  Number of nodes: {}", n_nodes);
    println!("  Edges:");
    for (i, v) in graph.iter().enumerate() {
        for j in v {
            println!("  {} {}", i, j);
        }
    }

    let components = kosajaru(graph, n_nodes);

    println!("Obtained result: ");

    for (node, component) in components.iter().enumerate() {
        println!("  node {} belongs to component {}", node, component);
    }

}
/*
    Given a graph represented by an adjacency list and the number of nodes, 
    return an array A such that A[i] == m iff the node i belongs to strongly 
    connected component m.

    The Kosajaru algorithm works by traversing the graph with DFS traversal and 
    storing in a stack the finalization order, from the node that ended first in the bottom
    of the stack, to the node that ended last, at the top of the  stack.

    Next, we have to traverse the inverse graph in the order stored in the stack, meaning
    that we traverse the nodes in the inverse finalization order. Each DFS tree generated by this
    second traversal represents a strongly connected component 
*/
fn kosajaru(graph : Graph, n_nodes : usize) -> Vec<usize> {

    // Order at which each node ends its DFS traversal
    let mut call_stack = vec![];
    // remember what nodes were visited so far with the memo array
    let mut memo = vec::from_elem(0, n_nodes);
    for node in 0..n_nodes {
        // visit all non-visited nodes
        if memo[node] == 0 {
            dfs_traversal(&graph, node, &mut memo, &mut call_stack)
        }
    }
    
    // Now that we have the call stack, we compute the strong components by traversing the 
    // inverse graph in the orden given by the call stack
    let graph_inverse = reverse_graph(&graph);
    
    // Reset our memo array
    for i in &mut memo{
        *i = 0;
    }
    
    // just a counter
    let mut numerator = 1..;
    for node in call_stack.iter().rev() {
        // traverse every non-visited node in the stack order (left to right in our stack vector)
        if memo[*node] == 0 {
            dfs_color(&graph_inverse, *node, &mut memo, numerator.next().unwrap());
        }
    }

    memo
}

// Utility function to reverse the given graph
fn reverse_graph(graph : &Graph) -> Graph{
    // init an empty graph
    let mut graph_inverse : Graph = vec::from_elem(vec![], graph.len());
    
    // iterate over our current graph, i is the current node, 
    // v is a list of nodes, such that there's an edge i -> j por every j in v.
    for (i, v) in graph.iter().enumerate() {
        for j in v {
            // we add an edge j -> i to our inverse graph 
            // for every edge i -> j in our graph
            graph_inverse[*j].push(i);
        }
    }   

    graph_inverse
}

// Given a graph, a source node v, the the list of not visited nodes 'memo' such that the node i 
// has not been visited iff memo[i] == 0, the stack of traversed nodes, 
// perform a dfs traversal over a graph storing the order at which each sub traversal has ended
// in the given stack
fn dfs_traversal(graph : &Graph, v : Node, memo : &mut Vec<Node>, stack : &mut Stack) {

    memo[v] = 1;
    for node in &graph[v] {
        if memo[*node] == 0 {
            dfs_traversal(graph, *node, memo, stack);
        }
    }
    stack.push(v);
}

// Traverse a graph starting in the node v and setting the color of the resulting
// dfs tree to the given color
fn dfs_color(graph : &Graph, v : Node, memo : &mut Vec<Node>, color : usize) {
    memo[v] = color;
    for node in &graph[v] {
        if memo[*node] == 0 {
            dfs_color(graph, *node, memo, color);
        }
    }
}


