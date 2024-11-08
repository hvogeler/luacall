Heiko = {
    last_name = "Vogeler",
    first_name = "Heiko",
    age = 62,
    home_owner = true,
}
Doris = {
    last_name = "Vogeler",
    first_name = "Doris",
    age = 65,
    home_owner = true,
}
Julius = {
    last_name = "Vogeler",
    first_name = "Julius",
    age = 32,
    home_owner = false,
}

Persons = { Heiko, Doris, Julius }

call_count = 0

function reformat (person)
    call_count = call_count + 1
    local interest_rate = 0.6
    if person.home_owner then
        interest_rate = 0.1
    end
    if person.age < 22 then
        interest_rate = 1
    end
    if person.age >=22 and person.age < 60 then
        interest_rate = interest_rate * .8
    end
    if person.age >= 60 and person.age < 75 then
        interest_rate = interest_rate * 1.3
        error("Too old person")
    end
    if person.age >=75 then
        interest_rate = .8
    end
    return {
        full_name = person.first_name .. " " .. person.last_name,
        interest_rate = interest_rate,
        call_count = call_count,
    }
end

function test1 ()
    for i, person in ipairs(Persons) do
        out = reformat(person)
        print(string.format("Interest rate for %s = %f", out.full_name, out.interest_rate))
    end
    print(Chris.first_name);
end

