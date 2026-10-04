import numpy as np

def monochrome(
    img: np.ndarray,
    blur_n: int = 3,
    window_radius: int = 3,
    min_prominence: float = 0.5,
    min_distance: int = 10,
    percentage: float = 0.22,
) -> np.ndarray: ...

__all__ = ["monochrome"]
