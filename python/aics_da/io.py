# -*- coding: utf-8 -*-

import numpy as np
import msgpack
from glob import glob


def load_msg(filename):
    with open(filename, "rb") as f:
        return msgpack.load(f)


def load_array(filename):
    with open(filename, "rb") as f:
        data = msgpack.load(f)
    return np.array(data[2]).reshape(data[1])


def load_arrays(filename):
    with open(filename, "rb") as f:
        data = msgpack.load(f)
    return np.array([np.array(d[2]).reshape(d[1]) for d in data])


def save_array(a, filename):
    data = (1, a.shape, a.flatten().tolist())
    with open(filename, "wb") as f:
        msgpack.dump(data, f)


def save_arrays(as_, filename):
    data = [(1, a.shape, a.flatten().tolist()) for a in as_]
    with open(filename, "wb") as f:
        msgpack.dump(data, f)


def load_ensembles(data_dir, return_length=False):
    T = len(glob(data_dir + "a*.msg"))

    def gen():
        for t in range(T):
            xs_b = load_arrays(data_dir + "b{:05d}.msg".format(t)).T
            xs_a = load_arrays(data_dir + "a{:05d}.msg".format(t)).T
            yield (xs_b, xs_a)
    if return_length:
        return T, gen()
    else:
        return gen()
