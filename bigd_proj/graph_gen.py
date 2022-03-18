import random
import networkx as nx

random.seed(10)

# an theloume na einai connected prepei afou tous dhmiourghsoume
# na tsekaroume to is_connected

n=10000 # number of nodes
d=100   # each node is connected to d nearest neighbors
p=0.45  # the probability of rewiring each edge
R=0.35  # distance threshold value
M=150   # the number of edges
p=0.3   # probability for edge creation

# uparxoun kai alles parametroi gia to grafhmata auta
# analutika sto documentation tou nx (kai exoun default times)
# px gia RGG uparxei to dim pou exei default 2 pou prosdiorizei ta dimensions tou graph



# uncomment 
graph = nx.watts_strogatz_graph(n, d, 0) #REG
# graph = nx.watts_strogatz_graph(n, d, p) #SW
# graph = nx.gnm_random_graph(n, M)        #RGER
# graph = nx.gnp_random_graph(n, p)        #RGGilbert
# graph = nx.random_geometric_graph(n, R)  #RGG
# graph = nx.barabasi_albert_graph(n, d)   #SF



res = 0
trian = nx.triangles(graph)
for i in trian:
  res += trian[i]
print(res/3)


count = 0
with open('random_graph.txt', 'w') as f:
    for line in nx.generate_edgelist(graph, data=["weight"]):
        if count != 0:
            f.write('\n')
        f.write(line)
        count += 1
    
