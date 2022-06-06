#!/bin/bash

set -x 
# gia na fainontai oi entoles

# cp random_graph_triangles_small.txt random_graph.txt
# cp random_graph_triangles_medium.txt random_graph.txt
# cp random_graph_triangles_big.txt random_graph.txt

# cp random_graph_degree_small.txt random_graph.txt
# cp random_graph_degree_medium.txt random_graph.txt
# cp random_graph_degree_big.txt random_graph.txt

# cp random_graph_pagerank_small.txt random_graph.txt
# cp random_graph_pagerank_medium.txt random_graph.txt
# cp random_graph_pagerank_big.txt random_graph.txt


# cp random_graph_triangles_small.txt random_graph.txt
# echo "******************************************";
# ./graph_changes.out 1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w6;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w12;
# echo "******************************************";
# ./graph_changes.out 5;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w6;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w12;
# echo "******************************************";
# ./graph_changes.out 10;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w6;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w12;
# 
# echo "******************************************";
# echo "******************************************";
# 
# cp random_graph_triangles_medium.txt random_graph.txt
# echo "******************************************";
# ./graph_changes.out 1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w6;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w12;
# echo "******************************************";
# ./graph_changes.out 5;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w6;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w12;
# echo "******************************************";
# ./graph_changes.out 10;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w6;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w12;
# 
# echo "******************************************";
# echo "******************************************";
# 
# cp random_graph_triangles_big.txt random_graph.txt
# echo "******************************************";
# ./graph_changes.out 1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w6;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w12;
# echo "******************************************";
# ./graph_changes.out 5;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w6;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w12;
# echo "******************************************";
# ./graph_changes.out 10;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w1;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w6;
# echo "******************************************";
# ./target/release/triangles 1 inspect -w12;
# echo "******************************************";
# 
# echo "******************************************";
# echo "******************************************";
# echo "******************************************";
# 
cp random_graph_pagerank_small.txt random_graph.txt
echo "******************************************";
./graph_changes.out 1;
echo "******************************************";
./target/release/pagerank 1 inspect -w1;
echo "******************************************";
./target/release/pagerank 1 inspect -w6;
echo "******************************************";
./target/release/pagerank 1 inspect -w12;
echo "******************************************";
./graph_changes.out 5;
echo "******************************************";
./target/release/pagerank 1 inspect -w1;
echo "******************************************";
./target/release/pagerank 1 inspect -w6;
echo "******************************************";
./target/release/pagerank 1 inspect -w12;
echo "******************************************";
./graph_changes.out 10;
echo "******************************************";
./target/release/pagerank 1 inspect -w1;
echo "******************************************";
./target/release/pagerank 1 inspect -w6;
echo "******************************************";
./target/release/pagerank 1 inspect -w12;

echo "******************************************";
echo "******************************************";

cp random_graph_pagerank_medium.txt random_graph.txt
echo "******************************************";
./graph_changes.out 1;
echo "******************************************";
./target/release/pagerank 1 inspect -w1;
echo "******************************************";
./target/release/pagerank 1 inspect -w6;
echo "******************************************";
./target/release/pagerank 1 inspect -w12;
echo "******************************************";
./graph_changes.out 5;
echo "******************************************";
./target/release/pagerank 1 inspect -w1;
echo "******************************************";
./target/release/pagerank 1 inspect -w6;
echo "******************************************";
./target/release/pagerank 1 inspect -w12;
echo "******************************************";
./graph_changes.out 10;
echo "******************************************";
./target/release/pagerank 1 inspect -w1;
echo "******************************************";
./target/release/pagerank 1 inspect -w6;
echo "******************************************";
./target/release/pagerank 1 inspect -w12;

echo "******************************************";
echo "******************************************";

cp random_graph_pagerank_big.txt random_graph.txt
echo "******************************************";
./graph_changes.out 1;
echo "******************************************";
./target/release/pagerank 1 inspect -w1;
echo "******************************************";
./target/release/pagerank 1 inspect -w6;
echo "******************************************";
./target/release/pagerank 1 inspect -w12;
echo "******************************************";
./graph_changes.out 5;
echo "******************************************";
./target/release/pagerank 1 inspect -w1;
echo "******************************************";
./target/release/pagerank 1 inspect -w6;
echo "******************************************";
./target/release/pagerank 1 inspect -w12;
echo "******************************************";
./graph_changes.out 10;
echo "******************************************";
./target/release/pagerank 1 inspect -w1;
echo "******************************************";
./target/release/pagerank 1 inspect -w6;
echo "******************************************";
./target/release/pagerank 1 inspect -w12;
echo "******************************************";

echo "******************************************";
echo "******************************************";
echo "******************************************";

cp random_graph_degree_small.txt random_graph.txt
echo "******************************************";
./graph_changes.out 1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w6;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w12;
echo "******************************************";
./graph_changes.out 5;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w6;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w12;
echo "******************************************";
./graph_changes.out 10;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w6;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w12;

echo "******************************************";
echo "******************************************";

cp random_graph_degree_medium.txt random_graph.txt
echo "******************************************";
./graph_changes.out 1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w6;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w12;
echo "******************************************";
./graph_changes.out 5;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w6;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w12;
echo "******************************************";
./graph_changes.out 10;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w6;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w12;

echo "******************************************";
echo "******************************************";

cp random_graph_degree_big.txt random_graph.txt
echo "******************************************";
./graph_changes.out 1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w6;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w12;
echo "******************************************";
./graph_changes.out 5;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w6;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w12;
echo "******************************************";
./graph_changes.out 10;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w1;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w6;
echo "******************************************";
./target/release/degree_centrality 1 inspect -w12;

set +x
# gia na epanerthoun ta swsta settings??
