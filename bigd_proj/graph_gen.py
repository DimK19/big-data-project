import random
import networkx as nx
from networkx.algorithms import bipartite
import time



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

# oi add_random kai remove_random dialegoun se kathe epanalhpsh sth tuxh ena komvo kai sth sunexeia sth tuxh ksana epilegoun apo th lista twn diathesimwn upoloipwn
# non_neighbors kai neighbors antistoixa ena epishs tuxaio komvo gia na prosthesoun h na afairesoun akmh. Se periptwsh pou den mporei na ginei kati tetoio auksanoun to err 
# kata ena to opoio an perasei ena threshold stamatei th diadiakasia kai ginontia mono oi prostheseis oi afaireseis pou eixan epiteuxthei mexri ekeino to vhma

def add_random_edges(graph, num = 1, threshold = 500):
    res = []
    res.append(str(num))
    count = 0
    err = 0
    while True:
        node1 = random.randrange(0,graph.number_of_nodes())
        # an valoume node + 1 edw mporoume na apofugoume ta self loops
        # node2 = random.randint(node1,graph.number_of_nodes()-1)
        temp = list(nx.non_neighbors(graph, node1))
        if len(temp) == 0:
            err += 1
        else:
            node2 = temp[random.randrange(len(temp))]
            count += 1
            if node1 < node2:
                res.append(str(node1) + " " + str(node2) + " 1")
                graph.add_edge(node1,node2)
            else:
                res.append(str(node2) + " " + str(node1) + " 1")
                graph.add_edge(node2,node1) 
        if count == num:
            return res
        elif err >= threshold:
            res[0] = str(count)
            print("Graph might be full. Difficulty adding any more edges. Added " + str(count) + " edges" )
            return res   

def remove_random_edges(graph, num = 1, threshold = 500):
    res = []
    res.append(str(num))
    count = 0
    err = 0
    while True:
        node1 = random.randrange(0,graph.number_of_nodes())
        # an valoume node + 1 edw mporoume na apofugoume ta self loops
        # node2 = random.randint(node1,graph.number_of_nodes()-1)
        temp = list(nx.neighbors(graph, node1))
        if len(temp) == 0:
            err += 1
        else:
            node2 = temp[random.randrange(len(temp))]
            count += 1
            if node1 < node2:
                res.append(str(node1) + " " + str(node2) + " -1")
                graph.remove_edge(node1,node2)
            else:
                res.append(str(node2) + " " + str(node1) + " -1")
                graph.remove_edge(node2,node1) 
        if count == num:
            return res
        elif err >= threshold:
            res[0] = str(count)
            print("Graph might be empty. Difficulty removing any more edges. Removed " + str(count) + " edges")
            return res

# to max_node xreiazetai etsi wste an o komvos den uparxei na dhmiourgei neo komvo wste na mporei na tou vazei tis katallhles akmes
# logika tha pairnei timh n pou einai to orisma gia to megethos kata th dhmiourgia tou grafhmatos
def add_node_edges(graph, node, max_node, num = -1):
    res = []
    if node >= max_node:
        graph.add_node(node)
    neig = list(nx.non_neighbors(graph, node))
    num_temp = len(list(neig))
    if num == -1:
        num = num_temp
    elif num > num_temp:
        num = num_temp
        print("Node " + str(node) + " does not have so many non neighbors")
        print("Adding " + str(num) + " edges instead")
    res.append(str(num))
    for i in range(num):
      neig = list(nx.non_neighbors(graph, node))
      rand_node = random.randrange(0, len(neig))
      if node < neig[rand_node]:
        res.append("" + str(node) + " " + str(neig[rand_node]) + " 1")
        graph.add_edge(node, neig[rand_node])
      else:
        res.append("" + str(neig[rand_node]) + " " + str(node) + " 1")
        graph.add_edge(node, neig[rand_node])


def remove_node_edges(graph, node, num = -1):
    res = []
    neig = list(nx.neighbors(graph, node))
    num_temp = len(neig)
    if num == -1:
        num = num_temp
    elif num > num_temp:
        num = num_temp
        print("Node " + str(node) + " does not have so many neighbors")
        print("Removing " + str(num) + " edges instead")
    res.append(str(num))
    for i in range(num):
      neig = list(nx.neighbors(graph, node))
      rand_node = random.randrange(0, len(neig))
      if node < neig[rand_node]:
        res.append("" + str(node) + " " + str(neig[rand_node]) + " -1")
        graph.remove_edge(node, neig[rand_node])
      else:
        res.append("" + str(neig[rand_node]) + " " + str(node) + " -1")
        graph.remove_edge(node, neig[rand_node])
    return res


def merge_rounds(res1, res2):
    res = []
    res.append(str(int(res1[0]) + int(res2[0])))
    for i in res1[1:]:
        res.append(i)
    for i in res2[1:]:
        res.append(i)
    return res


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


# to eixame gia th diarkeia twn dokimwn mporei na afairethei sth sunexeia
# h na meinei gia na uparxei omoiomorfia sta test an theloume
random.seed(10)

# genika to graph pou dhmiourgeitai ananewnetai meta apo kathe round etsi wste na exoun nohma kai oi allages pou ginontai sta epomena rounds

# an theloume na einai connected prepei afou tous dhmiourghsoume
# na tsekaroume to is_connected

n = 10000  # number of nodes
d = 50   # each node is connected to d nearest neighbors
p = 0.01  # the probability of rewiring each edge
R = 0.35  # distance threshold value
M = 150   # the number of edges
m = 10  # bipartite nodes of second side

# uparxoun kai alles parametroi gia to grafhmata auta
# analutika sto documentation tou nx (kai exoun default times)
# px gia RGG uparxei to dim pou exei default 2 pou prosdiorizei ta dimensions tou graph

start_time = time.time()

# uncomment
# graph = nx.watts_strogatz_graph(n, d, 0)                              #REG
# graph = nx.watts_strogatz_graph(n, d, p)                              #SW
# graph = nx.gnm_random_graph(n, M)                                     #RGER
# graph = nx.gnp_random_graph(n, p)                                     #RGGilbert (random)
# graph = nx.random_geometric_graph(n, R)                               #RGG
graph = nx.barabasi_albert_graph(n, d)                                #SF
# graph = bipartite.random_graph(n, m, p, seed=None, directed=False)    #bipartite


write_graph(graph)
print("--- %s seconds ---" % (time.time() - start_time))
#res1 = remove_node_edges(graph, return_centrality_nodes(graph, "degree", 1)[0][0])

# sto res2 afairoume komvo me mikro centrality apla oxi ton teleutaio gt meta apo elegxo den eixe akmes
#res2 = remove_node_edges(graph, return_centrality_nodes(graph, "closeness", 5, False)[4][0])

#res_merged = merge_rounds(res1, res2)

# h add kai h remove vazoun kai vgazoun tuxaies akmes enos komvou. An den epileksoume poses vazoun h vgazoun oles tis dunates
#Swrite_changes(res_merged + add_node_edges(graph, 1000, n, 10) +  remove_node_edges(graph, 0) + add_random_edges(graph, 300) + remove_random_edges(graph, 300))

write_changes([])
