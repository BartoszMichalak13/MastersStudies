# Bartosz Michalak
# Single machine scheduling - minimize weighted completion times

using JuMP
using GLPK
using MathOptInterface
using Test

const MOI = MathOptInterface

# Solver function
function solve_scheduling(p::Vector{Int}, w::Vector{Int}, r::Vector{Int})
  n = length(p)
  model = Model(GLPK.Optimizer)

  # Decision variables
  # Completion times
  @variable(model, C[1:n] >= 0)
  # Start times
  @variable(model, S[1:n] >= 0)
  # Ordering between tasks
  @variable(model, y[1:n, 1:n], Bin)

  M = sum(p) + maximum(r)

  for j in 1:n
    # Start time must be greater than or equal to Ready time
    @constraint(model, S[j] >= r[j])

    # Completion = Start + processing time
    @constraint(model, C[j] == S[j] + p[j])
  end

  for i in 1:n, j in 1:n
    if i != j
      # Ordering constraint
      # if task i is before task j, then S[j] must be greater than or equal to C[i]
      # otherwise, S[j] can be anything
      @constraint(model, S[j] >= C[i] - M * (1 - y[i, j]))
    end
  end

  for i in 1:n, j in 1:n
    if i != j
      # No cycles: for every pair, one must be first
      @constraint(model, y[i,j] + y[j,i] == 1)
    end
  end

  @objective(model, Min, sum(w[j] * C[j] for j in 1:n))

  optimize!(model)

  if termination_status(model) == MOI.OPTIMAL
    S_values = [value(S[j]) for j in 1:n]
    C_values = [value(C[j]) for j in 1:n]
    objective = objective_value(model)
    return S_values, C_values, objective
  else
    error("No optimal solution found.")
  end
end
