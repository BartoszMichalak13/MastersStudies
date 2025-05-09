# Bartosz Michalak
# Multimachine scheduling - minimize makespan

using JuMP
using Cbc

### User Data ###

# Each task is a tuple: (duration, resource requirements, predecessors)
tasks = [
    (50, [9], []),           # Task 1: duration 50, requires 9 units of resource, no predecessors
    (47, [17], [1]),         # Task 2: duration 47, needs 17 units, depends on Task 1
    (55, [11], [1]),         # Task 3: duration 55, needs 11 units, depends on Task 1
    (46, [4], [1]),          # Task 4: duration 46, needs 4 units, depends on Task 1
    (32, [13], [2]),         # Task 5: depends on Task 2
    (57, [7], [3, 4]),       # Task 6: depends on Tasks 3 and 4
    (15, [7], [4]),          # Task 7: depends on Task 4
    (62, [17], [5, 6, 7]),   # Task 8: depends on Tasks 5, 6, and 7
]

# Resource limits for each type (here only 1 type with a limit of 30)
r_max = [30]

### Data Preprocessing ###

n = length(tasks)                     # number of tasks
p = length(tasks[1][2])               # number of resource types

T = [task[1] for task in tasks]       # durations
R = [task[2] for task in tasks]       # resource demands
preds = [task[3] for task in tasks]   # predecessor lists

t_max = sum(T)                        # upper bound on makespan
F = t_max .- T .+ 1                   # latest possible start times per task

### Model ###

model = Model(Cbc.Optimizer)
set_optimizer_attribute(model, "threads", 12)
set_attribute(model, "logLevel", 0)

# Binary variables: x[j, t] = 1 if task j starts at time t
@variable(model, x[j in 1:n, t in 1:F[j]], Bin)

# Start time, finish time and cumulative resource usage
@variable(model, startTime[j in 1:n])                 # start time of task j
@variable(model, finishTime[j in 1:n])                # finish time of task j
@variable(model, demand[k in 1:p, t in 1:t_max])      # demand of resource k at time t
@variable(model, c_max >= 0)                          # makespan to be minimized

# Each task starts exactly once
@constraint(model, [j in 1:n], sum(x[j, t] for t in 1:F[j]) == 1)

# Link binary variables to start time
@constraint(model, [j in 1:n], startTime[j] == sum(x[j, t] * t for t in 1:F[j]))

# Finish time is start time plus duration minus 1
@constraint(model, [j in 1:n], finishTime[j] == startTime[j] + T[j] - 1)

# Compute total resource usage per resource per time unit
@constraint(model, [k in 1:p, t in 1:t_max],
  demand[k, t] == sum(
    x[j, r] * R[j][k]
    for j in 1:n, r in max(1, t - T[j] + 1):t if r <= F[j]
  )
)

# Makespan must be greater than or equal to finish time of all tasks
@constraint(model, [j in 1:n], finishTime[j] <= c_max)

# Precedence constraints: successors must start after predecessors finish
@constraint(model, [j in 1:n, b in preds[j]], startTime[j] >= finishTime[b] + 1)

# Capacity constraint: resource usage must not exceed limit
@constraint(model, [k in 1:p, t in 1:t_max], demand[k, t] <= r_max[k])

# Objective: minimize makespan
@objective(model, Min, c_max)

### Solution ###

optimize!(model)

status = termination_status(model)

if !is_solved_and_feasible(model)
  error("Solver did not find an optimal solution")
end

# Extract and round the solution
x_opt = value.(x)
s_opt = round.(Int, value.(startTime))    # task start times
f_opt = round.(Int, value.(finishTime))            # task finish times
d_opt = round.(Int, value.(demand))            # resource usage over time
c_max_opt = Int(objective_value(model))   # optimal makespan

# Padding width for Gantt chart
len = Int(ceil(log10(max(n, c_max_opt)))) + 3

# Output start and finish times
println(s_opt)
println(f_opt)
println(d_opt)

# Gantt-like visualization of task execution
for j in 1:n
  print("j", rpad(j, len, ' '))
  for t in 1:c_max_opt
    if t in s_opt[j]:f_opt[j]
      print('#'^len)
    else
      print(' '^len)
    end
  end
  println()
end

# Time scale
print("t", ' '^len)
for t in 1:c_max_opt
  print(rpad(t, len, ' '))
end
println()

# Resource usage at each time unit
print("r", ' '^len)
for k in 1:p
  for t in 1:c_max_opt
    print(rpad(d_opt[k, t], len, ' '))
  end
  println()
end

# Final result
println("makespan: $c_max_opt")
