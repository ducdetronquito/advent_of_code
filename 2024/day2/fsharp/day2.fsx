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
