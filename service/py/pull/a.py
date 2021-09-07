from ctypes import cdll



lib = cdll.LoadLibrary('./lib.so')

lib.test()