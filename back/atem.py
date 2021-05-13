from ctypes import *
# give location of dll
atem = cdll.LoadLibrary("C:\\Users\\assas\\Downloads\\8_3\\SwitcherLib.dll")
result1 = atem["GetSwitcherLibVersion"]
print "Addition value:"+result1
print "Substraction:"+result2