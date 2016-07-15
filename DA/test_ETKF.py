# -*- coding: utf-8 -*-

import numpy as np
from . import ETKF, observation, misc
from unittest import TestCase


class TestETKF(TestCase):

    def test_dimension(self):
        N = 40
        p = 20
        K = 10
        H = observation.head(N, p)
        xb = np.random.normal(size=N)
        Xb = np.array([xb + np.random.normal(size=N) for _ in range(K)]).T
        A = ETKF.analysis(H, np.identity(p))
        xa, Xa = A(xb, Xb, H(xb))


class TestETKF2(misc.TestLorenz96):

    def setUp(self):
        super().setUp(F=8, dt=0.01, N=40, T=1000, K=40)

    def test_assimilation(self):
        H = observation.trivial(self.N)
        R = np.identity(self.N)
        A = ETKF.analysis(H, R)
        obs = observation.add_noise(H, 1)
        rms = self.eval_rms(A, obs)
        self.assertLess(rms, 0.2)
