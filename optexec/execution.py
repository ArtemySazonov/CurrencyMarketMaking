import numpy as np
from numba import njit
from math import sqrt, sinh, cosh, acosh, log, exp

from dataloader import OnlineData

@njit
def sigma_2_rho(ret):
    return sqrt(np.var(ret))

class TWAP_model:
    """
    Time-Weighted Average Price strategy
    """  
    def __init__(self, T: int, L: int, W: np.ndarray) -> None:

        if T <= 0 or L <= 0:
            raise(ValueError)
        if L > T:
            raise(ValueError)
             
        self.T = T
        self.L = L 
        self.W = W
        self.num_of_rounds = W.shape[0]
        
        # idx = np.random.randint(low = 0, high = N, size = X - (X//N)*N)
        self.trading_list = (np.ones(shape = (self.num_of_rounds, L), dtype = int).T*(W//L)).T
        self.trading_list[:,-1]+=(W - (W//L)*L)

    def cumulative_impact(self, orderbook_bid: OnlineData, orderbook_ask: OnlineData):

        """ 
        Averaged Cost Per Round metric
        """
        ACPR = 0.
        dt = np.ediff1d(np.linspace(start = 0, stop = self.T, num = self.L, endpoint=True, dtype = int), to_begin=0)
        for rho in range(self.num_of_rounds):
            F2 = next(orderbook_ask)
            F1 = next(orderbook_bid)
            prW = (F2(1) + F1(1))//2 * self.W[rho]
            S = 0
            for l in range(0, self.L):
                for _ in range(dt[l]):
                    next(orderbook_bid)
                    next(orderbook_ask)
                S += iter(orderbook_bid).F(self.trading_list[rho, l])
            ACPR += (1 - S/(prW))
        return ACPR/self.num_of_rounds
            
    def reset(self):
        self.__init__(self.T, self.L, self.W)

@njit
def A_l_star(lamb, eta, sigma, W_rho, L, l , gamma = 0. ):
    tilde_kappa_2 = lamb * sigma /(eta - 0.5*gamma)
    kappa = acosh(0.5*tilde_kappa_2 + 1.)
    A = 2.*sinh(kappa/2.)*cosh(kappa * (L - l + 0.5) )*W_rho/ sinh(kappa * L)
    return A

class AC_model:
    def __init__(self, T: int, L: int, W: np.ndarray, lamb: float, eta: float, init_sigma: float, gamma: float = 0):
        if T <= 0 or L <= 0:
            raise(ValueError)
        if L > T:
            raise(ValueError)
        
        self.T = T
        self.L = L 
        self.W = W
        self.W_remain = np.copy(W)
        self.num_of_rounds = W.shape[0]
        self.tau = (1.*T)/L
        self.gamma = gamma
        self.eta = eta
        self.lamb = lamb
        self.init_sigma = init_sigma

    def cumulative_impact(self, orderbook_bid: OnlineData, orderbook_ask: OnlineData):

        """ 
        Averaged Cost Per Round metric
        """
        ACPR = 0.
        ret = np.empty(shape = self.num_of_rounds)
        dt = np.ediff1d(np.linspace(start = 0, stop = self.T, num = self.L, endpoint=True, dtype = int), to_begin=0)
        sigma = self.init_sigma

        for rho in range(self.num_of_rounds):
            F2 = next(orderbook_ask)
            F1 = next(orderbook_bid)
            pr1 = (F2(1) + F1(1))//2
            prW = pr1 * self.W[rho]
            S = 0

            for l in range(0, self.L - 1):

                for _ in range(dt[l]):
                    next(orderbook_bid)
                    next(orderbook_ask)

                Al = int(A_l_star(self.lamb, self.eta, sigma, self.W[rho], self.L, l + 1, self.gamma))
                S+=iter(orderbook_bid).F(Al)
                self.W_remain[rho] -= Al

            for _ in range(dt[-1]):
                next(orderbook_bid)
                next(orderbook_ask)

            Al = int(A_l_star(self.lamb, self. eta, sigma, self.W[rho], self.L, self.L, self.gamma))
            F1 = iter(orderbook_bid).F
            F2 = iter(orderbook_ask).F
            S += F1(self.W_remain[rho])
            self.W_remain[rho] = 0
            ACPR += (1 - S/(prW))
            prL = (F2(1) - F1(1))/2
            ret[rho] = log(prL/pr1)
            sigma = sigma_2_rho(ret[:rho + 1]) + 1e-8
        return ACPR/self.num_of_rounds
    
    def reset(self):
        self.__init__(self.T, self.L, self.W, self.lamb, self.eta,  self.init_sigma, self.gamma)


class GLOBE_model:
    """
    Greedy exploitation in Limit Order Book Execution

    """    
    def __init__(self, T: int, L: int, W: int, init_sigma: float, M: list = [], Mr: list = []) -> None:
        self.T = T
        self.L = L
        self.W = W
        self.M = M
        self.Mr = Mr
        self.num_or_rounds = W.shape[0]
        self.init_sigma = init_sigma


    def train(features: OnlineData):
        raise(NotImplementedError)

    def start():
        raise(NotImplementedError)

    def stop():
        raise(NotImplementedError)
    
    def reset(self):
        self.__init__(self.T, self.L, self.W, self.init_sigma, self.M, self.Mr)
