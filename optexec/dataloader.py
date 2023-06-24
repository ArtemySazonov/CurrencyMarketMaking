import numpy as np

class OnlineData:
    def __create_price_by_volume__(self):
        """Parses the data into the Callables

        Returns:
            _type_: _description_
        """
        raise(NotImplementedError)
        
    
    def __init__(self) -> None:
        self.file = None
        self.state = None

        raise(NotImplementedError)
    
    def update(self) -> None:

        raise(NotImplementedError)

    def copy_state(self):
        raise(NotImplementedError)