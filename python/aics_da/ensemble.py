# -*- coding: utf-8 -*-

import numpy as np
import scipy.stats
from . import linalg


def new(N, K):
    """ Create ensemble with zero mean

    Examples
    ---------
    >>> xs = new(10, 5)
    >>> xs.shape
    (5, 10)
    >>> np.allclose(average(xs), np.zeros(10))
    True
    >>> xs[0].shape
    (10,)
    """
    xs = np.random.normal(size=(K, N))
    xm = np.average(xs, axis=0)
    return xs - xm


def replica(x, K, noise=1.0):
    return x + noise*new(len(x), K)


def average(xs):
    return np.average(xs, axis=0)


def covar(xs):
    K, _ = xs.shape
    xm = average(xs)
    dxs = xs - xm
    return np.dot(dxs.T, dxs) / (K-1)


def deviations(xs):
    """ Get deviation vectors

    Examples
    ---------
    >>> xs = new(10, 5)
    >>> x, X = deviations(xs)
    >>> x.shape
    (10,)
    >>> np.allclose(x, np.zeros(10))
    True
    >>> X.shape
    (5, 10)
    >>> xs2 = reconstruct(x, X)
    >>> np.allclose(xs, xs2)
    True
    """
    xb = average(xs)
    return xb, xs - xb


def reconstruct(xb, Xb):
    return xb + Xb


def forcast(teo):
    def update(xs):
        for i, x in enumerate(xs):
            xs[i] = teo(x)
        return xs
    return update


def _sampling(cws, xs):
    """ Get sample from `xs` by with accumulated weight `cws` """
    return xs[np.searchsorted(cws, cws[-1]*np.random.random())]


def importance_sampling(ws, xs):
    """ execute importance sampling

    Parameters
    -----------
    ws : ndarray (1d)
        weights of particles
    xs : ndarray (2d)
        particles
    """
    if len(ws) != len(xs):
        raise RuntimeError(
            "Number of weights and particles mismatches: "
            "#ws={}, #xs={}".format(len(ws), len(xs))
        )
    cws = np.cumsum(ws)
    return np.array([_sampling(cws, xs) for _ in range(len(xs))])


def _gen_weight(n):
    """
    Generate weight for merge-resampling

    Examples
    ---------
    >>> a = _gen_weight(5)
    >>> np.allclose(np.sum(a), 1.0)
    True
    >>> np.allclose(np.sum(a**2), 1.0)
    True

    """
    a = np.random.random(n)
    M1 = np.sum(a[1:])
    M2 = np.sum(a[1:]**2)
    A = 2*M1/(M1**2 + M2)
    a *= A
    a[0] = (M2-M1**2) / (M2+M1**2)
    return a


def merge_resampling(ws, xs, n=3):
    """ execute merge resampling

    Parameters
    -----------
    ws : ndarray (1d)
        weights of particles
    xs : ndarray (2d)
        particles
    """
    if n < 2:
        raise RuntimeError(
            "Too small n for merge resampling: "
            "n={}".format(n)
        )
    cws = np.cumsum(ws)
    a = _gen_weight(n)
    return np.array([np.dot(a, [_sampling(cws, xs) for _ in range(n)])
                     for _ in range(len(xs))])


def Neff(ws):
    """ effective number of ensembles """
    return 1. / np.sum(ws**2)


def calc_weight(xs, y, H, Rinv):
    cs = np.array([linalg.quad(y-np.dot(H, x), Rinv)/2 for x in xs])
    return weight(cs)


def weight(cs):
    ws = np.exp(-cs)
    return ws / np.sum(ws)


def skewness(xs):
    """ skewness vector using 3rd order moment """
    dxs = xs - average(xs)
    try:
        J = np.linalg.inv(covar(xs))
    except np.linalg.LinAlgError:
        return np.zeros_like(xs[0])
    return np.einsum("ij,ti,tj,tk->k", J, dxs, dxs, dxs) / len(dxs)


def non_gaussianity(xs, l):
    xm = np.average(xs, axis=1)
    P = np.cov(xs)
    kernel = scipy.stats.gaussian_kde(xs)
    gaussian = scipy.stats.multivariate_normal(xm, P)
    ys = np.array([np.random.multivariate_normal(xm, P) for _ in range(l)]).T
    k = kernel.logpdf(ys)
    g = gaussian.logpdf(ys.T)
    return np.average(g - k)
