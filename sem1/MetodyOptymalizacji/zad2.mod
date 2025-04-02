/* Crane Transport Optimization */
/* Author: Bartosz Michalak */

/* Set of locations with demand and supply */
set Locations;

/* Demand for cranes of type I and II at each location */
param demand_I{Locations} >= 0;
param demand_II{Locations} >= 0;

/* Supply (excess) of cranes of type I and II at each location */
param supply_I{Locations} >= 0;
param supply_II{Locations} >= 0;

/* Distance matrix (km) between locations */
param dist{Locations, Locations} >= 0;

/* Cost per km for transporting one crane of type I */
param cost_I;

/* Cost per km for transporting one crane of type II (20% higher) */
param cost_II := 1.2 * cost_I;

/* Decision variables: Number of cranes transported */
var x_I{from in Locations, to in Locations} >= 0;
var x_II{from in Locations, to in Locations} >= 0;

/* Objective: Minimize transportation cost */
minimize Total_Cost:
    sum{from in Locations, to in Locations} (
        cost_I * dist[from, to] * x_I[from, to] +
        cost_II * dist[from, to] * x_II[from, to]
    );

/* Supply constraints: Cannot transport more cranes than available */
s.t. Supply_I{loc in Locations}:
    sum{to in Locations} x_I[loc, to] <= supply_I[loc];
s.t. Supply_II{loc in Locations}:
    sum{to in Locations} x_II[loc, to] <= supply_II[loc];

/* Demand constraints: Must satisfy demand at each location */
s.t. Demand_I{loc in Locations}:
    sum{from in Locations} x_I[from, loc] + sum{from in Locations} x_II[from, loc] - demand_II[loc] >= demand_I[loc];
s.t. Demand_II{loc in Locations}:
    sum{from in Locations} x_II[from, loc] - sum{from in Locations} x_I[from, loc] + demand_I[loc] >= demand_II[loc];

solve;

printf "Crane Transport Plan:\n";
for {from in Locations, to in Locations: x_I[from, to] > 0}
    printf "Move %.2f cranes of type I from %s to %s\n", x_I[from, to], from, to;
for {from in Locations, to in Locations: x_II[from, to] > 0}
    printf "Move %.2f cranes of type II from %s to %s\n", x_II[from, to], from, to;

printf "Total cost: %.2f\n", Total_Cost;

end;
