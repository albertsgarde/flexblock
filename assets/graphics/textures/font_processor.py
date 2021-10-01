# -*- coding: utf-8 -*-
"""
Created on Thu Sep 30 21:44:03 2021

@author: Carl
"""


import matplotlib.pyplot as plt
import numpy as np

img = plt.imread("gui.png")
uvs = np.zeros((256,4))
for x in range(0,16):
    for y in range(0,16):
        letter = img[y*16:y*16+16,x*16:x*16+16,3]
        if np.any(letter>0.3):
            vmin = np.min(np.argwhere(letter > 0.3)[:,0])
            umin = np.min(np.argwhere(letter > 0.3)[:,1])
            vmax = np.max(np.argwhere(letter > 0.3)[:,0])
            umax = np.max(np.argwhere(letter > 0.3)[:,1])
            uvs[x+y*16,:] = np.array([umin,vmin,umax,vmax],dtype=int)

np.savetxt("font_info.csv", uvs, fmt='%i', delimiter=",")