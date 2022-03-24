import random
import networkx as nx
from networkx.algorithms import bipartite


def test_print(graph, type = "pagerank"):
    if type == "pagerank":
        print(nx.pagerank(graph))
    elif type == "triangles":
        res = 0
        trian = nx.triangles(graph)
        for i in trian:
          res += trian[i]
        print(res/3) 
    elif type == "degree_centrality":
        print(nx.degree_centrality(graph))
    elif type == "bipartite":
        print(bipartite.is_bipartite(graph))
    else:
        print("You should provide type: pagerank(default), triangles, degree centrality, bipartite")

def return_centrality_nodes(graph, centrality = "degree", num = 5, top = True):
    if centrality == "degree":
        res = nx.res = nx.degree_centrality(graph)
    elif centrality == "closeness":
        res = nx.res = nx.closeness_centrality(graph)
    elif centrality == "betweeness":
        res = nx.res = nx.betweenness_centrality(graph)
    else:
        print("You should provide type: pagerank(default), triangles, degree centrality, bipartite")
        exit 
    if len(res.items()) < num :
        num = len(res.items())
        print(num)
    return sorted(res.items(), key=lambda x: x[1], reverse=top)[:num]

def return_node_neighbors(graph, node):
    return list(graph.adj[node])

def remove_node_edges(graph, node, num = -1):
    res = []
    neig = list(graph.adj[node])
    num_temp = len(list(graph.adj[node]))
    if num == -1:
        num = num_temp
    elif num > num_temp:
        num = num_temp
        print("Node " + str(node) + " does not have so many neighbors")
    res.append(str(num))
    for i in neig[:num]:
      if node < i:
        res.append("" + str(node) + " " + str(i) + " -1")
        graph.remove_edge(node, i)
      else:
        res.append("" + str(i) + " " + str(node) + " -1")
        graph.remove_edge(node, i)
    return res

def add_random_edges(graph, num = 1, threshold = 50):
    res = []
    res.append(str(num))
    count = 0
    err = 0
    while True:
        node1 = random.randint(1,graph.number_of_nodes()-1)
        # an valoume node + 1 edw mporoume na apofugoume ta self loops
        node2 = random.randint(node1,graph.number_of_nodes()-1)
        if graph.has_edge(node1,node2):
            err += 1
        else:
            count += 1
            res.append(str(node1) + " " + str(node2) + " 1")
            graph.add_edge(node1,node2)
        if count == num:
            return res
        elif err >= threshold:
            print("Graph might be full. Difficulty adding any more edges.")
            exit   

def remove_random_edges(graph, num = 1, threshold = 50):
    res = []
    res.append(str(num))
    count = 0
    err = 0
    while True:
        node1 = random.randint(1,graph.number_of_nodes()-1)
        # an valoume node + 1 edw mporoume na apofugoume ta self loops
        node2 = random.randint(node1,graph.number_of_nodes()-1)
        if not graph.has_edge(node1,node2):
            err += 1
        else:
            count += 1
            res.append(str(node1) + " " + str(node2) + " -1")
            graph.remove_edge(node1,node2)
        if count == num:
            return res
        elif err >= threshold:
            print("Graph might be empty. Difficulty removing any more edges.")
            exit



def write_changes(res):
    count = 0
    with open('edges_to_change.txt', 'w') as f:
        for line in res:
            if count != 0:
                f.write('\n')
            f.write(line)
            count += 1


def write_graph(graph):
    count = 0
    with open('random_graph.txt', 'w') as f:
        for line in nx.generate_edgelist(graph, data=["weight"]):
            if count != 0:
                f.write('\n')            
            f.write(line)
            count += 1

random.seed(10)

# an theloume na einai connected prepei afou tous dhmiourghsoume
# na tsekaroume to is_connected

n = 1000  # number of nodes
d = 100   # each node is connected to d nearest neighbors
p = 0.55  # the probability of rewiring each edge
R = 0.35  # distance threshold value
M = 150   # the number of edges
m = 10  # bipartite nodes of second side

# uparxoun kai alles parametroi gia to grafhmata auta
# analutika sto documentation tou nx (kai exoun default times)
# px gia RGG uparxei to dim pou exei default 2 pou prosdiorizei ta dimensions tou graph


# uncomment
# graph = nx.watts_strogatz_graph(n, d, 0)                              #REG
graph = nx.watts_strogatz_graph(n, d, p)                              #SW
# graph = nx.gnm_random_graph(n, M)                                     #RGER
# graph = nx.gnp_random_graph(n, p)                                     #RGGilbert
# graph = nx.random_geometric_graph(n, R)                               #RGG
# graph = nx.barabasi_albert_graph(n, d)                                #SF
# graph = bipartite.random_graph(n, m, p, seed=None, directed=False)    #bipartite


write_graph(graph)

res = remove_node_edges(graph, return_centrality_nodes(graph, "degree", 1)[0][0])

write_changes(res + remove_node_edges(graph, 0) + add_random_edges(graph, 5) + remove_random_edges(graph, 2))