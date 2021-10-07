import quadkeys
import math

EARTH_RADIUS = 6378137
LATITUDE_RANGE = (-85.05112878, 85.05112878)


def clip(n, min_max):
    return min(max(n, min_max[0]), min_max[1])


def map_size(level):
    base = 256
    return base << level


def ground_resolution(lat, level):
    lat = clip(lat, LATITUDE_RANGE)
    return math.cos(lat * math.pi / 180) * 2 * math.pi * EARTH_RADIUS / map_size(level)


def test_pure_python(benchmark):
    benchmark(ground_resolution, 90, 10)


def test_rust(benchmark):
    benchmark(quadkeys.py_ground_resolution, 90, 10)
