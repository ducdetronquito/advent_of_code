open System.IO
open System

type Report = int array

let parse (filePath: string) : Report seq =
    let readLines filePath = File.ReadLines(filePath)

    let parseLine (line: string) : string array =
        line.Split([| " " |], StringSplitOptions.None)

    let parseRow (row: string array) = row |> Array.map int

    filePath |> readLines |> Seq.map parseLine |> Seq.map parseRow


let levelDifferences (report: Report) : int list =
    report
    |> Array.toList
    |> List.windowed 2
    |> List.map (fun (window) -> window[0] - window[1])

let allLevelIncrease (levelDifferences: int list) : bool =
    let increaseCount =
        levelDifferences
        |> List.sumBy (fun levelDifference -> if levelDifference > 0 then 1 else 0)

    increaseCount = levelDifferences.Length

let allLevelDecrease (levelDifferences: int list) : bool =
    let increaseCount =
        levelDifferences
        |> List.sumBy (fun levelDifference -> if levelDifference < 0 then 1 else 0)

    increaseCount = levelDifferences.Length


let allLevelDifferenceAreLow (levelDifferences: int list) : bool =
    let lowDifferenceCount =
        levelDifferences
        |> List.map abs
        |> List.sumBy (fun absoluteLevelDifference ->
            if absoluteLevelDifference >= 1 && absoluteLevelDifference <= 3 then
                1
            else
                0)

    lowDifferenceCount = levelDifferences.Length

let reportIsSafe (report: Report) : bool =
    let levelDifferences = levelDifferences report

    let allLevelIncrease = allLevelIncrease levelDifferences
    let allLevelDecrease = allLevelDecrease levelDifferences

    let allDifferencesAreLow = allLevelDifferenceAreLow levelDifferences

    allDifferencesAreLow && (allLevelIncrease || allLevelDecrease)

let exampleReports = parse "2024/day2/example.txt"


let safeExampleReports =
    exampleReports
    |> Seq.map reportIsSafe
    |> Seq.sumBy (fun reportSafety -> if reportSafety = true then 1 else 0)


printfn "Number of safe example reports: %i" safeExampleReports
// Expect 2


let inputReports = parse "2024/day2/input.txt"

let safeInputReports =
    inputReports
    |> Seq.map reportIsSafe
    |> Seq.sumBy (fun reportSafety -> if reportSafety = true then 1 else 0)


printfn "Number of safe example reports: %i" safeInputReports
// Expect 598


// ----- Part 2 -----


// Given a array, popProduct returns all possible arrays
// where a single element of the input array was removed
// Ex:
// input = [|1; 2; 3; 4|]
// Output = [[|1; 2; 3|]; [|1; 2; 4|]; [|1; 3; 4|]; [|2; 3; 4|];]
let popProduct (input: 'a array) : 'a array list =
    let rec popProductImpl (input: 'a array) (indexToPop: int) (acc: 'a array list) : 'a array list =
        if indexToPop = input.Length - 1 then
            acc
        else
            let nextIndexToPop = indexToPop + 1
            let product = Array.concat [ input[0..indexToPop]; input[nextIndexToPop + 1 ..] ]
            popProductImpl (input) (indexToPop + 1) (product :: acc)

    popProductImpl input -1 []


let reportIsSafeish (report: Report) : bool =
    let pontentialCorrectedReports = popProduct report

    report :: pontentialCorrectedReports
    |> List.map reportIsSafe
    |> List.contains true


let safeishExampleReports =
    exampleReports
    |> Seq.sumBy (fun reportSafety -> if reportIsSafeish reportSafety then 1 else 0)

printfn "Number of safe-ish example reports: %i" safeishExampleReports
// Expect 4

let safeishInputReports =
    inputReports
    |> Seq.sumBy (fun reportSafety -> if reportIsSafeish reportSafety then 1 else 0)

printfn "Number of safe-ish input reports: %i" safeishInputReports
// Expect 634
