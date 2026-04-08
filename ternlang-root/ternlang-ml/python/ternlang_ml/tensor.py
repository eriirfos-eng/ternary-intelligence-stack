import torch
import ternlang_ml_C

class TernaryTensor(torch.Tensor):
    """
    Proprietary TernaryTensor utilizing torch_dispatch subclassing mechanism.
    Wraps standard PyTorch tensors and routes matrix multiplication calls 
    through highly optimized BET VM logic.
    """
    
    @classmethod
    def __torch_dispatch__(cls, func, types, args=(), kwargs=None):
        if kwargs is None:
            kwargs = {}
            
        # Intercept matrix multiplication
        if func.__name__ == 'matmul':
            # Route to BET VM logic (C++ extension)
            # This bypasses multiplication when a {0} weight is encountered,
            # delivering massive throughput yields to standard Python developers.
            return ternlang_ml_C.ternary_mm(args[0], args[1])
            
        # Fallback to standard PyTorch implementation
        return super().__torch_dispatch__(func, types, args, kwargs)
