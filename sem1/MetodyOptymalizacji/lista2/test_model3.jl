# Bartosz Michalak
# Multimachine scheduling - minimize makespan
using Test
include("model3.jl")  # Load our solver

@testset "Scheduling Tests" begin

  @testset "Example from textbook" begin
    p = [1, 2, 1, 2, 1, 1, 3, 6, 2]
    pred::Dict{Int64, Vector{Int64}} = Dict(4=>[1,2,3], 5=>[2,3], 6=>[4], 7=>[4,5], 8=>[5], 9=>[6,7])
    m = 3
    result = solve_schedule(p, pred, m)
    @test result.status == MOI.OPTIMAL
    @test result.Cmax ≈ 9 atol=1e-3
  end

  @testset "Simple linear chain, 1 machine" begin
    p = [2, 2, 2]
    pred::Dict{Int64, Vector{Int64}} = Dict(2 => [1], 3 => [2])
    m = 1
    result = solve_schedule(p, pred, m)
    @test result.status == MOI.OPTIMAL
    @test result.Cmax ≈ 6 atol=1e-3
  end

  @testset "Parallel tasks, no dependencies" begin
    p = [3, 3, 3]
    pred::Dict{Int64, Vector{Int64}} = Dict()
    m = 3
    result = solve_schedule(p, pred, m)
    @test result.status == MOI.OPTIMAL
    @test result.Cmax ≈ 3 atol=1e-3
  end

  @testset "Single machine, all independent" begin
    p = [1, 2, 3]
    pred::Dict{Int64, Vector{Int64}} = Dict()
    m = 1
    result = solve_schedule(p, pred, m)
    @test result.status == MOI.OPTIMAL
    @test result.Cmax ≈ 6 atol=1e-3
  end

end
