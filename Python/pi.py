#!/usr/bin/env python
import time
pairs = [10_000_000, 100_000_000]
for p in pairs:
   sum = 0.0
   beg = time.time()
   for i in range(p):
      j = i * 4.0 
      sum += 2.0 / ((j + 1.0) * (j + 3.0))
   end = time.time()
   print(f"{4.0 * sum} in {end - beg} sec. for {p} pairs")

