open System.IO
open System

let parse (filePath: string) : (int * int) seq =
    let readLines filePath = File.ReadLines(filePath)

    let parseLine (line: string) : string array =
        line.Split([| "   " |], StringSplitOptions.None)

    let parseRow (row: string array) : (int * int) = (int row[0], int row[1])

    filePath |> readLines |> Seq.map parseLine |> Seq.map parseRow


let getDistance (positions: (int * int) seq) : int =
    let leftPositions = positions |> Seq.map fst |> Seq.sort
    let rightPositions = positions |> Seq.map snd |> Seq.sort


    let distance =
        Seq.zip leftPositions rightPositions
        |> Seq.map (fun (left, right) -> abs (left - right))
        |> Seq.sum

    distance


let examplePositions = parse "2024/day1/example.txt"

printfn "Distance of example: %i" (getDistance examplePositions)
// Expect 11

let inputPositions = parse "2024/day1/input.txt"

printfn "Distance of example: %i" (getDistance inputPositions)
// Expect 2196996

// ----- Part 2 -----

let getSimilarityScore (positions: (int * int) seq) : int =
    let leftPositions = positions |> Seq.map fst
    let rightPositions = positions |> Seq.map snd |> Seq.toList

    let rightPositionOccurences = rightPositions |> Seq.countBy id |> Map

    let similartyScore (leftPosition: int) =
        let occurences =
            rightPositionOccurences.TryFind(leftPosition) |> Option.defaultValue 0

        leftPosition * occurences


    leftPositions |> Seq.map similartyScore |> Seq.sum


printfn "Similarity score of example: %i" (getSimilarityScore examplePositions)
// Expect 31

printfn "Similarity score of example: %i" (getSimilarityScore inputPositions)
// Expect 23655822
