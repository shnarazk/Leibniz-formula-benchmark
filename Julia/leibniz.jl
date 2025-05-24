#!/usr/bin/env julia
#
function pi(limit::Int64)::Float64
    sum::Float64 = 0.0
    for i in 0:limit
        k::Float64 = 4.0 * i
        # sum += (-1)^i / (2i + 1)
        sum += 2.0 / ((k + 1.0) * (k + 3.0))
    end
    4.0 * sum
end

num_pairs = 100_000_000
start_time = time()
p = pi(num_pairs)
end_time = time()
elapsed_time = end_time - start_time
println("Execution time: $elapsed_time seconds for $num_pairs : $p")
