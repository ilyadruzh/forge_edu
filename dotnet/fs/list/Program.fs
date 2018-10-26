// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    printfn "Hello World from F#!"
    0 // return an integer exit code

let rec insertions x = function
    [] -> [[x]]
  | h::t -> (x::h::t)::(map (fun z -> h::z)(insertions x t));;  