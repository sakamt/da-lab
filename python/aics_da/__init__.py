
import numpy as np
import scipy.stats
import msgpack


def load_msg(filename):
    with open(filename, "rb") as f:
        data = msgpack.load(f)
    return np.array(data[2]).reshape(data[1])


def save_msg(a, filename):
    data = (1, a.shape, a.flatten().tolist())
    with open(filename, "wb") as f:
        msgpack.dump(data, f)


def non_gaussianity(xs, l):
    xm = np.average(xs, axis=1)
    P = np.cov(xs)
    kernel = scipy.stats.gaussian_kde(xs)
    gaussian = scipy.stats.multivariate_normal(xm, P)
    ys = np.array([np.random.multivariate_normal(xm, P) for _ in range(l)]).T
    k = kernel.logpdf(ys)
    g = gaussian.logpdf(ys.T)
    return np.average(g - k)
