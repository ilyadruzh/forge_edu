// Learn more about F# at http://fsharp.org

open System

[<EntryPoint>]
let main argv =
    printfn "Hello World from F#!"
    0 // return an integer exit code

let mandelf (c:Complex) (z:Complex) = z*z+c;;

let ismandel c = Complex.Abs(repeatN 20 (mandelf c) (Complex.zero))<1.0;;

let rec forl a b f = 
  if a>=b then f(b)
  else 
   begin f(a); forl (a+1) b f end ;;

let scale (x:float,y:float) (u,v) n = float(n-u)/float(v-u)*(y-x)+x;;

forl 1 60 (fun i ->
 forl 1 60 (fun j ->
   let lscale = scale (-1.2,1.2) (1,60) in
   let t = complex (lscale j) (lscale i) in 
   Console.Write(if ismandel t then "*" else " ")
 );
 Console.WriteLine("")
);;