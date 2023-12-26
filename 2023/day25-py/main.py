import networkx as nx
import sys

G = nx.Graph()

for line in sys.stdin:
    line = line.strip()
    node, edges = line.split(':')
    for edge in [x.strip() for x in edges.strip().split(' ')]:
        G.add_edge(node, edge)

G.remove_edges_from(nx.minimum_edge_cut(G))
l, r = [len(c) for c in nx.connected_components(G)]
print(f"silver: {l * r}")
print(f"gold: {l * r}")

