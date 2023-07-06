import dataloader
import execution

import unittest
import numpy as np

class TestTWAP(unittest.TestCase):
    def setUp(self):
        self.strategy1 = execution.TWAP(1, 100, 100)
        self.strategy2 = execution.TWAP(1, 1003, 100)
        self.strategy3 = execution.TWAP(1, 100, 1003)

    def test_initialization(self):
        """
            Test: executed volume is equal to the initial one
        """        

        assert np.sum(self.strategy1.trading_list) == self.strategy1.X
        assert np.sum(self.strategy2.trading_list) == self.strategy2.X
        assert np.sum(self.strategy3.trading_list) == self.strategy3.X
    

if __name__ == "__main__":
    unittest.main()
