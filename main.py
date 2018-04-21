from typing import NamedTuple, List

import numpy as np


class Node(NamedTuple):
    x: float
    y: float
    z: float
    a: float
    id: int


class Edge(NamedTuple):
    node1: int
    node2: int


