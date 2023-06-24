import numpy as np

from dataloader import OnlineData

class TWAP():
    """Time-Weighted Average Price strategy

    """  
    def __init__(self, T: float, N: int, X: int) -> None:
        if T <= 0 or N <= 0:
            raise(ValueError)
             
        self.T = T
        self.N = N
        self.X = X
        self.X_remaining = X
        self.clock = 0
        self.tau = T/N
        
        idx = np.random.randint(low = 0, high = N, size = X - int(X/N)*N)
        self.trading_list = np.ones(N)*int(X/N)
        self.trading_list[idx]+=1

    def cumulative_impact(orderbook: OnlineData):
        raise(NotImplementedError)

    def reset(self):
        self.__init__(self.T, self.N, self.X)
    

class GLOBE():
    """Greedy exploitation in Limit Order Book Execution

    """    
    def __init__(self, T: float, N: int, X: int) -> None:
        super().__init__(T, N, X)

    def train(features: OnlineData):
        raise(NotImplementedError)

    def start():
        raise(NotImplementedError)

    def stop():
        raise(NotImplementedError)
    
    def reset(self):
        self.__init__(self.T, self.N, self.X)
