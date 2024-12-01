open System.IO
open System


let getDistance (positions: (int * int) seq) : int =
    let leftPositions = positions |> Seq.map fst |> Seq.sort
    let rightPositions = positions |> Seq.map snd |> Seq.sort


    let distance =
        Seq.zip leftPositions rightPositions
        |> Seq.map (fun (left, right) -> abs (left - right))
        |> Seq.sum

    distance


let parse (filePath: string) : (int * int) seq =
    let readLines filePath = File.ReadLines(filePath)

    let parseLine (line: string) : string array =
        line.Split([| "   " |], StringSplitOptions.None)

    let parseRow (row: string array) : (int * int) = (int row[0], int row[1])

    filePath |> readLines |> Seq.map parseLine |> Seq.map parseRow


let examplePositions = [ 3, 4; 4, 3; 2, 5; 1, 3; 3, 9; 3, 3 ]

assert (getDistance examplePositions = 11)


let inputPositions = parse "../input-part1.txt"

assert (getDistance inputPositions = 2196996)
