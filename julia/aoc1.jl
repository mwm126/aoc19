function main()
    words = readlines("input-1.txt")
    masses = map(word->parse(Int32, word), words)
    fuels = map(fuel, masses)
    print("\nTotal fuel requirement: ", sum(fuels))

    fuels = map(fuel2, masses)
    print("\nTotal fuel requirement: ", sum(fuels))
end

function fuel(mass)
    mass ÷ 3 - 2
end

function fuel2(mass)
    f = fuel(mass)
    if f<=0
        return 0
     else
        f + fuel2(f)
    end
end

main()