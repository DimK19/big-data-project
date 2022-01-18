import random
import networkx as nx

random.seed(10)

n=10
d=5
p=0.25

# για την κατασκευή του SW χρησιμοποιήστε την watts_strogatz_graph από το networkx. 
#uncomment 
SW = nx.watts_strogatz_graph(n, d, p)
res = 0
trian = nx.triangles(SW)
for i in trian:
    res += trian[i]
print(res/3)
count = 0
with open('random_graph.txt', 'w') as f:
    for line in nx.generate_edgelist(SW, data=["weight"]):
        if count != 0:
            f.write('\n')
        f.write(line)
        count += 1
    