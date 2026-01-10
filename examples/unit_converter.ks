function to_meters(inches float): float {
    return inches / 39.37f;
}

function to_inches(meters float): float {
    return meters * 39.37f;
}

let result = to_inches(10f);

println(result);
