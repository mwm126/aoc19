function main()
    words = readlines("input-1.txt")
    masses = map(word->parse(Int32, word), words)
    fuels = map(fuel, masses)
    print("Total fuel requirement: ", sum(fuels))
end

function fuel(mass)
    mass ÷ 3 - 2
end

main()