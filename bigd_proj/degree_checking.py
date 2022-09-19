import random
import networkx as nx
from networkx.algorithms import bipartite
import time
import matplotlib.pyplot as plt
import numpy as np

def find_degree(G, show = 1):

    res = nx.degree_centrality(G)
    if show != 0:
      plt.title("Degree centrality")
      plt.hist(res.values(), bins = 10)
      #plt.bar(res.keys(),res.values())
      plt.show()

      print("mean value: " + str(np.mean(list(res.values()))))
      print("variance: " + str(np.var(list(res.values()))))

    return res

g = nx.read_edgelist("random_graph.txt",create_using=nx.Graph(), nodetype = int)

print(g.number_of_nodes())

find_degree(g)

# an den paizei swsta o kwdikas autos mporei na thelei na treksei se jupyter notebook (copy paste se kapoio keli)
