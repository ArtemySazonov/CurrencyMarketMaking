import dataloader
import execution

import unittest
import numpy as np

class TestTWAP(unittest.TestCase):
    def setUp(self):
        self.strategy = execution.TWAP(1, 100, 100)

    def test_initialization(self):
        """
            Test: executed volume is equal to the initial one
        """        
        assert np.sum(self.strategy.trading_list) == self.strategy.X
    

if __name__ == "__main__":
    unittest.main()
