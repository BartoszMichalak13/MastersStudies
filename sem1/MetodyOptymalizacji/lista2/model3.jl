# Bartosz Michalak
# Multimachine scheduling - minimize makespan
using JuMP
using GLPK

function solve_schedule(p::Vector{Int}, pred::Dict{Int, Vector{Int}}, m::Int)
  n = length(p)
  model = Model(GLPK.Optimizer)

  @variable(model, 0 <= s[1:n])                     # start times
  @variable(model, x[1:n, 1:m], Bin)                # machine assignment
  @variable(model, Cmax >= 0)                       # makespan
  @variable(model, delta[1:n, 1:n], Bin)            # disjunctive order

  big_M = sum(p)

  # Each task is assigned to exactly one machine
  for i in 1:n
    @constraint(model, sum(x[i, k] for k in 1:m) == 1)
  end

  # Precedence constraints
  for j in 1:n
    for i in get(pred, j, [])
      @constraint(model, s[j] >= s[i] + p[i])
    end
  end

  # Disjunctive constraints (no overlap on the same machine)
  for i in 1:n, j in i+1:n, k in 1:m
    @constraint(model, s[i] + p[i] <= s[j] + big_M * (1 - delta[i, j] + 1 - (x[i,k] + x[j,k] - 1)))
    @constraint(model, s[j] + p[j] <= s[i] + big_M * (delta[i, j] + 1 - (x[i,k] + x[j,k] - 1)))
  end

  # Order decision: for all i ≠ j, delta[i,j] + delta[j,i] == 1
  for i in 1:n, j in 1:n
    if i != j
      @constraint(model, delta[i, j] + delta[j, i] == 1)
    end
  end

  # Makespan constraints
  for j in 1:n
    @constraint(model, s[j] + p[j] <= Cmax)
  end

  @objective(model, Min, Cmax)
  optimize!(model)

  # Extract machine assignments from x[i,k]
  machine_assignment = [findfirst(k -> value(x[i, k]) > 0.5, 1:m) for i in 1:n]

  print_schedule(p, value.(s), machine_assignment)

  return (
    status = termination_status(model),
    Cmax = objective_value(model),
    start_times = value.(s),
    machines = machine_assignment
  )
end

function print_schedule(p::Vector{Int}, start_times::Vector{Float64}, machines::Vector{Int})
  n = length(p)
  tasks = [(i, start_times[i], p[i], machines[i]) for i in 1:n]
  sorted_tasks = sort(tasks, by = x -> x[4])  # sort by machine

  println("\nTextual Gantt chart:")

  time_horizon = Int(ceil(maximum(start_times .+ p)))

  for m in 1:maximum(machines)
    line = "Machine $m: "
    timeline = fill("   ", time_horizon + 1)

    for (i, s, dur, mach) in sorted_tasks
      if mach == m
        for t in Int(floor(s)):Int(floor(s + dur - 1))
          timeline[t + 1] = lpad(string(i), 3)
        end
      end
    end

    println(line * join(timeline))
  end
end
