"""
This script plot the graph given by resuCities.dat and resuGraph.dat
"""

from matplotlib import pyplot
from numpy import loadtxt

CITIES = loadtxt("resuCities.dat", dtype=float, delimiter=" ")
pyplot.scatter(CITIES [:, 1], CITIES [:, 2], s=CITIES [:, 0]/1000, c=CITIES [:, 0], alpha=0.5)

GRAPH = loadtxt("resuGraph.dat", dtype=int)
for x in range(GRAPH.shape[0]):
    edge = [GRAPH[x, 0], GRAPH[x, 1]]
    pyplot.plot(CITIES[edge, 1], CITIES[edge, 2], 'b')

pyplot.xlabel('Longitude', size=16)
pyplot.ylabel('Latitude', size=16)

# To save in a PNG file:
pyplot.savefig('result.png', dpi=1080)

# To show:
pyplot.show()
