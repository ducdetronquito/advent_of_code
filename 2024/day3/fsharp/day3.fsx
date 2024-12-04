open System.IO
open System.Text.RegularExpressions

let pattern =
    Regex(@"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)", RegexOptions.Compiled)


type Instruction =
    | Do
    | Dont
    | Multiply of int * int

let parse (input: string) : Instruction list =
    seq {
        for m in pattern.Matches(input) do
            match m.Groups[0].Value with
            | "do()" -> Do
            | "don't()" -> Dont
            | _ -> Multiply(int m.Groups[1].Value, int m.Groups[2].Value)
    }
    |> Seq.toList


let compute (input: Instruction list) : int =
    input
    |> List.choose (fun instruction ->
        match instruction with
        | Multiply(a, b) -> Some(a * b)
        | _ -> None)
    |> List.sum


let preciseCompute (instructions: Instruction list) : int =
    let rec doCompute (instructions: Instruction list) (enabled: bool) (output: int) : int =
        match instructions with
        | [] -> output
        | Do :: rest -> doCompute rest true output
        | Dont :: rest -> doCompute rest false output
        | Multiply(a, b) :: rest ->
            match enabled with
            | true -> doCompute rest true (output + a * b)
            | false -> doCompute rest false output

    doCompute instructions true 0


let instructions = "2024/day3/input.txt" |> File.ReadAllText |> parse

let resultPart1 = instructions |> compute
let resultPart2 = instructions |> preciseCompute
