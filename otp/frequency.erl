-module(frequency).
-export([start/0, stop/0, allocate/0, deallocate/1]).
-export([init/0, terminate/1, handle/2]).

start() ->
    server:start(frequency, []).

init(_Args) ->
    {get_frequencies(), []}.

get_frequencies() -> [10, 11, 12, 13, 14, 15].

stop() -> server:stop(frequency).
allocate() -> server:call(frequency, {allocate, self()}).
deallocate(Freq) -> server:call(frequency, {deallocate, Freq}).

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

terminate(_Frequencies) ->
    ok.

handle({allocate, Pid}, Frequencies) ->
    allocate(Frequencies, Pid);

handle({deallocate, Freq}, Frequencies) -> 
    {deallocate(Frequencies, Freq), ok}.

allocate({[], Allocated}, _Pid) ->
    {{[], Allocated}, {error, no_frequecy}};

allocate({[Freq|Free], Allocated}, Pid) ->
    {{Free, [{Freq, Pid}|Allocated]}, {ok, Freq}}.

deallocate({Free, Allocated}, Freq) ->
    NewAllocated = lists:keydelete(Freq, 1, Allocated),
    {[Freq|Free], NewAllocated}.