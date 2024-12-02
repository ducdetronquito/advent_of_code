open System.IO
open System

type Report = int array

let parse (filePath: string) : Report seq =
    filePath
    |> File.ReadLines
    |> Seq.map (_.Split([| " " |], StringSplitOptions.None) >> Array.map int)


let rec all (matcher: 'a -> bool) (items: 'a list) : bool =
    match items with
    | [] -> true
    | head :: rest when matcher head -> all matcher rest
    | _ -> false

let countIf (matcher: 'a -> bool) (items: 'a seq) : int =
    items
    |> Seq.map matcher
    |> Seq.sumBy (fun value -> if value = true then 1 else 0)

let levelDifferences (report: Report) : int list =
    report
    |> Array.toList
    |> List.pairwise
    |> List.map (fun (first, second) -> first - second)


let reportIsSafe (report: Report) : bool =
    let levelDifferences = levelDifferences report

    let allLevelIncrease = levelDifferences |> all (fun item -> item > 0)
    let allLevelDecrease = levelDifferences |> all (fun item -> item < 0)

    let allDifferencesAreLow =
        levelDifferences |> List.map abs |> all (fun value -> value >= 1 && value <= 3)

    allDifferencesAreLow && (allLevelIncrease || allLevelDecrease)

let exampleReports = parse "2024/day2/example.txt"


let safeExampleReports = exampleReports |> countIf reportIsSafe


printfn "Number of safe example reports: %i" safeExampleReports
// Expect 2

let inputReports = parse "2024/day2/input.txt"

let safeInputReports = inputReports |> countIf reportIsSafe

printfn "Number of safe example reports: %i" safeInputReports
// Expect 598

// ----- Part 2 -----
let removeOneLevel (report: Report) : Report list =
    [ for i in 0 .. report.Length do
          Array.concat [ report[0 .. i - 1]; report[i + 1 ..] ] ]


let reportIsSafeish (report: Report) : bool =
    report |> removeOneLevel |> List.map reportIsSafe |> List.contains true


let safeishExampleReports = exampleReports |> countIf reportIsSafeish


printfn "Number of safe-ish example reports: %i" safeishExampleReports
// Expect 4

let safeishInputReports = inputReports |> countIf reportIsSafeish

printfn "Number of safe-ish input reports: %i" safeishInputReports
// Expect 634
