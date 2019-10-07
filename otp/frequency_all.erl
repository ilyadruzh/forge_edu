-module(frequency).
-export([start/0, stop/0, allocate/0, deallocate/1]).
-export([init/0]).

start() ->
    register(frequency, spawn(frequency, init, [])).

init() ->
    Frequencies = {get_frequencies(), []},
    loop(Frequencies).

get_frequencies() -> [10, 11, 12, 13, 14, 15].

stop() -> call(stop).

allocate() -> call(allocate).

deallocate(Freq) -> call({deallocate, Freq}).

call(Message) ->
    frequency ! {request, self(), Message},
    receive {reply, Reply} -> Reply end.

reply(Pid, Reply) -> Pid ! {reply, Reply}.

loop(Frequencies) ->
    receive
        {request, Pid, allocate} ->
            {NewFrequenciesm, Reply} = allocate(Frequencies, Pid),
            reply(Pid, Reply),
            loop(NewFrequenciesm);
        {request, Pid, {deallocate, Freq}} ->
            NewFrequenciesm = deallocate(Frequencies, Freq),
            reply(Pid, ok),
            loop(NewFrequenciesm);
        {request, Pid, stop} -> reply(Pid, ok)
    end.