# Bartosz Michalak
# Cutting Stock Problem - Minimize Total Leftover

using JuMP
using Cbc
using MathOptInterface  # For MOI.OPTIMAL

const MOI = MathOptInterface

# Load data
include("dane1.jl")

# Function to generate all possible cutting patterns
function generate_patterns(STANDARD_WIDTH, DEMAND)
  widths = collect(keys(DEMAND))
  patterns = []

  function search(current_pattern, remaining_width)
    if remaining_width < minimum(widths)
      push!(patterns, current_pattern)
      return
    end
    for w in widths
      if w <= remaining_width
        search(vcat(current_pattern, w), remaining_width - w)
      end
    end
  end

  search(Int[], STANDARD_WIDTH)
  return patterns
end

# Generate cutting patterns
patterns = generate_patterns(STANDARD_WIDTH, DEMAND)

println("Generated ", length(patterns), " cutting patterns.")

# Precompute wastes (leftover width for each pattern)
wastes = [STANDARD_WIDTH - sum(pattern) for pattern in patterns]

# Create model
model = Model(Cbc.Optimizer)

# Decision variables: how many boards cut according to each pattern
@variable(model, x[1:length(patterns)] >= 0, Int)

# Decision variables: excess pieces for each width
@variable(model, excess[1:length(patterns)] >= 0, Int)

# Constraints: meet demand for each width
for (width, quantity) in DEMAND
  @constraint(model, sum(count(==(width), patterns[i]) * x[i] for i in 1:length(patterns)) == quantity + excess[width])
end

# Objective: minimize total leftover
@objective(model, Min, sum(wastes[i] * x[i] for i in 1:length(patterns)) + sum(excess[width] * width for width in keys(DEMAND)))

# Solve the model
optimize!(model)

# Display results
if termination_status(model) == MOI.OPTIMAL
  println("\nOptimal solution found:")
  println("Total leftover minimized: ", objective_value(model))
  println("\nCutting patterns used:")
  for i in 1:length(patterns)
    if value(x[i]) > 0.5
      println("Pattern ", i, ": ", patterns[i], " -> used ", round(Int, value(x[i])), " times. Pattern waste: ", wastes[i])
    end
  end
  println("Total standard boards used: ", sum(value(x[i]) for i in 1:length(patterns)))
else
  println("No optimal solution found.")
end
