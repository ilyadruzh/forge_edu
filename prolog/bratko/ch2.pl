f(1, one).
f(s(1), two).
f(s(s(1)), three).
f(s(s(s(X))), N) :-
    f(X, N).

transform(Num, Word) :-
    Num=1,
    Word=one. 
transform(Num, Word) :-
    Num=2,
    Word=two.
transform(Num, Word) :-
    Num=3,
    Word=three.

big(bear).
big(elephant).
small(cat).
black(cat).
brown(bear).
gray(elephant).
dark(Z) :-
    black(Z).
dark(Z) :-
    brown(Z).

state(-, -, -, have).
step(state(middle, onbox, middle, havenot), grab, state(middle, onbox, middle, have)).
step(state(P1, onfloor, B, H), move(P1, P2), state(P2, onfloor, B, H)).
step(state(P, onfloor, P, H), climb, state(P, onbox, P, H)).
step(state(P1, onfloor, P1, H), reloc(P1, P2), state(P2, onfloor, P2, H)).
% подвинуть, залезть
cantake(state(-,-,-, have)).
cantake(State1) :- step(State1, Step, State2), cantake(State2).

