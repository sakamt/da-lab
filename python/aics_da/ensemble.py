
import numpy as np
import scipy.stats


def non_gaussianity(xs, l):
    xm = np.average(xs, axis=1)
    P = np.cov(xs)
    kernel = scipy.stats.gaussian_kde(xs)
    gaussian = scipy.stats.multivariate_normal(xm, P)
    ys = np.array([np.random.multivariate_normal(xm, P) for _ in range(l)]).T
    k = kernel.logpdf(ys)
    g = gaussian.logpdf(ys.T)
    return np.average(g - k)
