import random
import networkx as nx

random.seed(10)

# an theloume na einai connected prepei afou tous dhmiourghsoume
# na tsekaroume to is_connected

n=100 # number of nodes
d=5   # each node is connected to d nearest neighbors
p=0.45  # the probability of rewiring each edge
R=0.35  # distance threshold value
M=150   # the number of edges
p=0.3   # probability for edge creation

# uparxoun kai alles parametroi gia to grafhmata auta
# analutika sto documentation tou nx (kai exoun default times)
# px gia RGG uparxei to dim pou exei default 2 pou prosdiorizei ta dimensions tou graph


# uncomment 
# graph = nx.watts_strogatz_graph(n, d, 0) #REG
# graph = nx.watts_strogatz_graph(n, d, p) #SW
# graph = nx.gnm_random_graph(n, M)        #RGER
# graph = nx.gnp_random_graph(n, p)        #RGGilbert
# graph = nx.random_geometric_graph(n, R)  #RGG
graph = nx.barabasi_albert_graph(n, d)   #SF


# print triangles for testing purposes
# res = 0
# trian = nx.triangles(graph)
# for i in trian:
#   res += trian[i]
# print(res/3)

# print pagerank for testing purposes
# print(nx.pagerank(graph))

# print degrees for testing purposes
# print(nx.degree_centrality(graph))


# treis diaforetikes kentrikothtes se periptwsh pou theloume stoxeumenous komvous
# uparxei kai h kentrikothta katz an theloume alla einai ligo pio mperdemenh
# uncomment
# res = nx.degree_centrality(graph)
# res = nx.closeness_centrality(graph)
# res = nx.betweenness_centrality(graph)

# etsi kratame tous n top komvous gia kathe kentrikothta
# uncomment
# n = 5
# top = sorted(res.items(), key=lambda x: x[1], reverse=True)[:n]

# emfanish olwn twn geitonwn tou komvou n
n = 0
print(list(graph.adj[n]))

# tupwma olwn twn akmwn stis opoies uparxei o komvos n
n = 0
count = 0;
for i in list(graph.adj[n]):
  if count != 0:
    print("\n", end ="")
  if n<i:
    print(str(n) + " "+ str(i), end ="")
  else:
    print(str(i) + " "+ str(n), end ="")
  count += 1
# auto einai ena newline gia to terminal
print("")


count = 0
with open('random_graph.txt', 'w') as f:
    for line in nx.generate_edgelist(graph, data=["weight"]):
        if count != 0:
            f.write('\n')
        f.write(line)
        count += 1