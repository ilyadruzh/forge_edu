-module(server).
-export([start/2, stop/0, call/2]).
-export([init/2]).

start(Name, Args) ->
    register(Name, spawn(server, init, [Name, Args])).

init(Mod, Args) ->
    State = Mod:init(Args),
    loop(Mod, State).

get_frequencies() -> [10, 11, 12, 13, 14, 15].

stop(Name) -> 
    Name ! {stop, self()},
    receive {reply, Reply} -> Reply end.

allocate() -> call(allocate).

deallocate(Freq) -> call({deallocate, Freq}).

call(Message) ->
    frequency ! {request, self(), Message},
    receive {reply, Reply} -> Reply end.

call(Name, Msg) ->
    Name ! {request, self(), Msg},
    receive {reply, Reply} -> Reply end.

reply(To, Reply) -> 
    To ! {reply, Reply}.

loop(Mod, State) ->
    receive
        {request, From, Msg} ->
            {NewState, Reply} = Mod:handle(Msg, State),
            reply(From, Reply),
            loop(Mod, NewState);
        {stop, From} -> 
            Reply = Mod:terminate(State),
            reply(From, Reply)
    end.