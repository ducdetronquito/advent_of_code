open System.IO

type PageNumber = int
type OrderingRules = Map<PageNumber, PageNumber list>
type PageSet = PageNumber list

let parseOrderingRules (path: string) : OrderingRules * PageSet list =
    let content = File.ReadLines path |> Seq.toList
    let sep = content |> Seq.findIndex (fun row -> row.Length = 0)

    let rules =
        content[0 .. sep - 1]
        |> List.map (fun row ->
            let items = row.Split("|")
            int (items[0]), int (items[1]))
        |> List.groupBy (fun item -> fst item)
        |> List.map (fun (key, groups) -> (key, groups |> List.map snd))
        |> Map.ofList

    let pageSets =
        content[sep + 1 ..]
        |> List.map (fun row ->
            [ for item in row.Split(",") do
                  int item ])

    rules, pageSets

let pageSetHasCorrectOrder (orderingRules: OrderingRules) (pageSet: PageNumber list) : bool =
    pageSet
    |> Seq.mapi (fun index pageNumber ->
        let pagesBefore = pageSet[0 .. index - 1]
        (pageNumber, pagesBefore))
    |> Seq.map (fun (pageNumber, pagesBefore) ->
        match (pagesBefore, orderingRules.TryFind pageNumber) with
        | [], _ -> true
        | _, None -> true
        | _, Some(rules) -> Set.intersect (set pagesBefore) (set rules) |> Set.isEmpty)
    |> Seq.filter (fun correctOrder -> correctOrder = false)
    |> Seq.isEmpty


let getCorrectPageSets (path: string) : PageSet list =
    let rules, pagesToProduce = path |> parseOrderingRules

    pagesToProduce
    |> List.filter (fun pageSet -> pageSetHasCorrectOrder rules pageSet)


let getResult (correctPageSets: PageSet list) : int =
    let middlePageNumber (pageSet: PageNumber list) : PageNumber =
        if pageSet.Length % 2 = 0 then
            pageSet[pageSet.Length / 2]
        else
            pageSet[(pageSet.Length - 1) / 2]

    correctPageSets |> List.map middlePageNumber |> List.sum


"2024/day5/example.txt" |> getCorrectPageSets |> getResult // Expect 143

"2024/day5/input.txt" |> getCorrectPageSets |> getResult // Expect 4689
