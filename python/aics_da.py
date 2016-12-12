
import numpy as np
import msgpack


def load_msg(filename):
    with open(filename, "rb") as f:
        data = msgpack.load(f)
    return np.array(data[2]).reshape(data[1])


def save_msg(a, filename):
    data = (1, a.shape, a.flatten().tolist())
    with open(filename, "wb") as f:
        msgpack.dump(data, f)
