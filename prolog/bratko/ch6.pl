cube :- read(X), resol(X).
resol(stop).
resol(N) :- C is N * N * N, write(C), cube.