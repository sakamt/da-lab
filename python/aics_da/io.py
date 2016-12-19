# -*- coding: utf-8 -*-

import numpy as np
import msgpack


def load_array(filename):
    with open(filename, "rb") as f:
        data = msgpack.load(f)
    return np.array(data[2]).reshape(data[1])


def load_arrays(filename):
    with open(filename, "rb") as f:
        data = msgpack.load(f)
    return [np.array(d[2]).reshape(d[1]) for d in data]


def save_array(a, filename):
    data = (1, a.shape, a.flatten().tolist())
    with open(filename, "wb") as f:
        msgpack.dump(data, f)


def save_arrays(as_, filename):
    data = [(1, a.shape, a.flatten().tolist()) for a in as_]
    with open(filename, "wb") as f:
        msgpack.dump(data, f)
