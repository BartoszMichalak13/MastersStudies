#= 
Autor: Bartosz Michalak
=#

using JuMP
using HiGHS
using LinearAlgebra
using Graphs
using Printf

decode_instance(filename) = begin
  open(filename, "r") do file
    n, m, _ = parse.(Int, split(readline(file)))
    _ = readline(file)  # Skip comment/blank line
    p = Matrix{Int}(undef, n, m)
    for i in 1:n
      full_row = parse.(Int, split(readline(file)))
      if length(full_row) != 2m
        error("Wiersz $i ma długość $(length(full_row)), oczekiwano 2m = $(2m).")
      end
      p[i, :] = full_row[2:2:end]
    end
    return n, m, p  # jobs, machines, matrix
  end
end

# Greedy harmonogram: przypisuje każde zadanie do najszybszej maszyny
greedy_makespan(p) = begin
  n, m = size(p)
  assign = [argmin(p[i, :]) for i in 1:n]
  loads = zeros(Int, m)
  for j in 1:n
    i = assign[j]
    loads[i] += p[j, i]
  end
  return maximum(loads)
end

# function build_lp_model(p::Matrix{Int}, T::Float64)
function build_lp_model(p::Matrix{Int}, T::Int)
  jobs_num, machines_num = size(p)
  model = Model(HiGHS.Optimizer)
  set_silent(model)

  S_T_complement = [(i,j) for i in 1:jobs_num, j in 1:machines_num if p[i, j] > T]
  S_T_i = [[j for j in 1:machines_num if p[i, j] <= T] for i in 1:jobs_num]
  S_T_j = [[i for i in 1:jobs_num if p[i, j] <= T] for j in 1:machines_num]

  # x[i, j] = 1 if job i is assigned to machine j, 0 otherwise
  @variable(model, x[1:jobs_num, 1:machines_num] >= 0) # relaxation to allow fractional assignments

  # use only the pairs (i, j) where processing time is less than or equal to T
  @constraint(model, [(i, j) in S_T_complement], x[i, j] == 0)

  # each job must be assigned to exactly one machine
  @constraint(model, [i in 1:jobs_num], sum(x[i, j] for j in S_T_i[i]) == 1)

  # total processing time on each machine j must not exceed T
  @constraint(model, [j in 1:machines_num], sum(p[i, j] * x[i, j] for i in S_T_j[j]) <= T)

  # check only if model is feasible
  @objective(model, Min, 0)
  return model, x
end

# Szukanie minimalnego T*, dla którego LP(T*) ma rozwiązanie
function find_T_star(p::Matrix{Int}, a::Int)
  n, m = size(p)
  low, high = Int(floor(a / m)), a
  bestT = high
  while high - low > 1
    mid = Int(round((low + high) / 2))
    model, _ = build_lp_model(p, mid)
    optimize!(model)
    if termination_status(model) == MOI.OPTIMAL
      bestT = mid
      high = mid
    else
      low = mid
    end
  end
  return bestT
end

# Ekstremalne rozwiązanie: LP daje zawsze ekstremalne przy użyciu solvera simplex
function extract_solution(model, x)
  return value.(x)
end


function assign_fractional_jobs(x::Matrix{Float64}, p::Matrix{Int})
  n, m = size(p)
  F = [j for j in 1:n if any(!(isapprox(x[j, i], 0.0; atol=1e-7) || isapprox(x[j, i], 1.0; atol=1e-7)) for i in 1:m)]
  adj = Dict{Int, Vector{Int}}()
  for j in F
    adj[j] = [i for i in 1:m if !(isapprox(x[j, i], 0.0; atol=1e-7) || isapprox(x[j, i], 1.0; atol=1e-7))]
  end
  # for j in F
  #   println("Fractional job $j is assigned to machines: ", adj[j], " with x values: ", x[j, adj[j]])
  # end
  # for j in F
  #   for i in 1:m
  #     if j <= 200
  #       println("x[$j, $i] = $(x[j, i])")
  #     end
  #   end
  # end
  match_to_machine = Dict{Int, Int}()  # job → machine
  match_to_job = fill(-1, m)

  function bpm(j::Int, visited::Vector{Bool})::Bool
    for i in adj[j]
      if !visited[i]
        visited[i] = true
        if match_to_job[i] == -1 || bpm(match_to_job[i], visited)
          match_to_job[i] = j
          match_to_machine[j] = i
          return true
        end
      end
    end
    return false
  end

  for j in F
    visited = fill(false, m)
    success = bpm(j, visited)
    if !success
      # for j in 1:n
      #   for i in 1:m
      #     print("x[$j, $i] = $(x[j, i]), ")
      #   end
      #   println()
      # end
      for i in 1:m
        println("Machine $i: x[$j, $i] = $(x[j, i])")
      end
      error("Matching failed for fractional job $j — H has no perfect matching")
    end
  end

  return match_to_machine
end

# Główna funkcja harmonogramowania
function schedule(p::Matrix{Int})
  a = greedy_makespan(p)
  T_star = find_T_star(p, a)
  model, xvar = build_lp_model(p, T_star)
  optimize!(model)
  xsol = extract_solution(model, xvar)

  n, m = size(p)
  final_assign = fill(-1, n)
  for j in 1:n
    for i in 1:m
      if isapprox(xsol[j,i], 1.0; atol=1e-7)
        final_assign[j] = i
        break
      end
    end
  end

  unassigned = findall(==( -1 ), final_assign)
  # println("Unassigned jobs: ", unassigned)
  # for j in unassigned
  #   println("Job $j x[]: ", xsol[j, :])
  # end
  matching = assign_fractional_jobs(xsol, p)

  count = 0
  for j in unassigned
    if haskey(matching, j)
      final_assign[j] = matching[j]
      count += 1
    else
      error("Fractional job $j could not be matched to any machine!")
    end
  end

  # Oblicz ostateczny makespan
  loads = zeros(Int, m)
  for j in 1:n
    i = final_assign[j]
    loads[i] += p[j, i]
  end
  return final_assign, maximum(loads), T_star, count
end


if abspath(PROGRAM_FILE) == @__FILE__
  using Dates
  using Printf

  if length(ARGS) != 1
    println("Użycie: julia twój_plik.jl ścieżka_do_folderu/")
    exit(1)
  end

  folder = ARGS[1]
  txt_files = filter(f -> endswith(f, ".txt"), readdir(folder, join=true))
  output_filename = "wyniki_$(basename(folder)).txt"

  open(output_filename, "w") do out
    for file in txt_files
      println("Plik: $(basename(file))")
      println(out, "Plik: $(basename(file))")

      # Start pomiaru czasu
      start_time = time()
      n, m, p = decode_instance(file)
      assign, makespan, T_star, fractional_jobs = schedule(p)
      elapsed = time() - start_time

      # Oblicz całkowitoliczbowe przypisania
      integer_jobs = (length(assign) - fractional_jobs)
      percent_integer = 100 * integer_jobs / length(assign)
      # Zapisz wyniki
      println(out, @sprintf("  Liczba zadań: %d", n))
      println(out, @sprintf("  Liczba maszyn: %d", m))
      println(out, @sprintf("  Makespan: %d", makespan))
      println(out, @sprintf("  T*: %.2f", T_star))
      println(out, @sprintf("  Czas działania (s): %.4f", elapsed))
      println(out, @sprintf("  Całkowitoliczbowe przypisania: %d (%.2f%%)", integer_jobs, percent_integer))
      println(out, "  Przypisania: $(assign)")
      println(out, "\n" * "-"^80 * "\n")
    end
  end

  println("Wyniki zapisano do pliku: $output_filename")
end
