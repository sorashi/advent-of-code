import networkx as nx
import sys

G = nx.Graph()

for line in sys.stdin:
    line = line.strip()
    node, edges = line.split(':')

    for edge in [x.strip() for x in edges.strip().split(' ')]:
        G.add_edge(node, edge)

cut = nx.minimum_edge_cut(G)
G.remove_edges_from(cut)
c = [len(c) for c in nx.connected_components(G)]
print(f"silver: {c[0] * c[1]}")

