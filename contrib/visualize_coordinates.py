import matplotlib.pyplot as plt

mapRect = [(-30720, -43008), (30720, 43008)]
continentRect = [(5630, 11518), (8190, 15102)]

coordsHillsContinent = (8086.65, 13520.8)
coordsBayContinent = (5867.06, 13596.6)
coordsBayGarrContinent = (6886.26, 13075.6)
coordsLeftSpawnCampContinent = (6090.42, 14093.9)


def convert(contCoords):
    normalized = (
        (contCoords[0] - continentRect[0][0]) / (continentRect[1][0] - continentRect[0][0]),
        (contCoords[1] - continentRect[0][1]) / (continentRect[1][1] - continentRect[0][1])
    )
    return (
        (mapRect[1][0] - mapRect[0][0]) * normalized[0] + mapRect[0][0],
        (mapRect[1][1] - mapRect[0][1]) * normalized[1] + mapRect[0][1]
    )


coordsHillsMap = convert(coordsHillsContinent)
coordsBayMap = convert(coordsBayContinent)
coordsGarrMap = convert(coordsBayGarrContinent)
coordsLeftSpawnCampMap = convert(coordsLeftSpawnCampContinent)
coordsMeMap = (-19622.90, -18841.50)

data = [
    (
        coordsHillsMap[0],
        coordsHillsMap[1],
        'Hills', 'red'),
    (
        coordsBayMap[0],
        coordsBayMap[1],
        'Bay', 'red'),
    (
        coordsGarrMap[0],
        coordsGarrMap[1],
        'Garr', 'red'),
    (
        coordsLeftSpawnCampMap[0],
        coordsLeftSpawnCampMap[1],
        'Left Spawn Camp', 'red'),
    (
        coordsMeMap[0],
        -coordsMeMap[1],
        'ME', 'green')
]

mapEdges = [
    (
        -30720,
        -43008, 'Map edge A', 'blue'),
    (
        30720,
        43008, 'Map edge B', 'blue')
]

x, y, names, colors = zip(*(mapEdges + data))

plt.scatter(x, y, color=colors)

for i in range(len(x)):
    plt.text(x[i], y[i], '  ' + names[i])

plt.gca().invert_yaxis()

plt.xlabel('X-axis')
plt.ylabel('Y-axis')
plt.title('WvW Map')
plt.show()
