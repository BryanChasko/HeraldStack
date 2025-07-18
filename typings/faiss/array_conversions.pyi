"""
This type stub file was generated by pyright.
"""

from faiss.loader import *

sizeof_long = ...
deprecated_name_map = ...
vector_name_map = ...
def vector_to_array(v): # -> _Array1D[float64] | ndarray[tuple[int], dtype[Any]]:
    """ convert a C++ vector to a numpy array """
    ...

def vector_float_to_array(v): # -> _Array1D[float64] | ndarray[tuple[int], dtype[Any]]:
    ...

def copy_array_to_vector(a, v): # -> None:
    """ copy a numpy array to a vector """
    ...

def copy_array_to_AlignedTable(a, v): # -> None:
    ...

def array_to_AlignedTable(a): # -> AlignedTableUint16 | AlignedTableUint8:
    ...

def AlignedTable_to_array(v): # -> _Array1D[float64]:
    """ convert an AlignedTable to a numpy array """
    ...

