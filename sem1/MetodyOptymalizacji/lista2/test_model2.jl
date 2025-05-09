# Bartosz Michalak
# Single machine scheduling - minimize weighted completion times

using Test
include("model2.jl")  # Load our solver

@testset "Single machine scheduling tests" begin
  @testset "Simple 4-task example" begin
    p = [3, 2, 4, 1]
    w = [2, 1, 3, 4]
    r = [0, 1, 2, 0]
    S, C, obj = solve_scheduling(p, w, r)

    @test all(S[j] >= r[j] for j in 1:4)  # All start times respect release dates
    @test all(C[j] == S[j] + p[j] for j in 1:4)  # Completion = start + processing time

    # Manually verified best objective value
    @test round(obj, digits=2) == 46
  end

  @testset "Trivial case with no release times" begin
    p = [2, 1, 3]
    w = [1, 2, 3]
    r = [0, 0, 0]
    S, C, obj = solve_scheduling(p, w, r)

    @test all(S[j] >= 0 for j in 1:3)
    @test all(C[j] == S[j] + p[j] for j in 1:3)

    # Quick check that objective value is positive
    @test obj > 0
  end

  @testset "All tasks available at different times" begin
    p = [5, 3, 2]
    w = [4, 1, 2]
    r = [0, 5, 10]
    S, C, obj = solve_scheduling(p, w, r)

    @test S[1] >= 0
    @test S[2] >= 5
    @test S[3] >= 10
    @test all(C[j] == S[j] + p[j] for j in 1:3)
  end

  @testset "Another Testcase" begin
    p = [3, 4, 7, 5, 3]
    w = [2, 2, 3, 5, 10]
    r = [1, 2, 2, 2, 6]
    S, C, obj = solve_scheduling(p, w, r)

    @test round(obj, digits=2) == 267
  end

  @testset "Another Testcase" begin
    p = [1]
    w = [1]
    r = [1]
    S, C, obj = solve_scheduling(p, w, r)

    @test round(obj, digits=2) == 2
  end

  @testset "Tablica" begin
    p = [3,2,4,5,1]
    w = [1,1,1,1,1]
    r = [2,1,3,1,0]
    S, C, obj = solve_scheduling(p, w, r)

    @test round(obj, digits=2) == 35
  end
end
