/* Hilbert Matrix LP Problem - Task 1 */
/* Author: Bartosz Michalak */

/* Parameter: Problem size n */
param n, integer, > 0;

/* Define the Hilbert Matrix A (n x n) */
param A{i in 1..n, j in 1..n} := 1 / (i + j - 1);

/* Define cost vector c and right-hand side vector b */
param c{i in 1..n} := sum{j in 1..n} (1 / (i + j - 1));
param b{i in 1..n} := sum{j in 1..n} (1 / (i + j - 1));

/* Decision variables */
var x{i in 1..n} >= 0;

/* Objective function: minimize c^T * x */
minimize obj: sum{i in 1..n} c[i] * x[i];

/* Constraints: Ax = b */
s.t. equality{i in 1..n}: sum{j in 1..n} A[i,j] * x[j] = b[i];

solve;

printf "Solution for n = %d\n", n;
for {i in 1..n} printf "x[%d] = %.6f\n", i, x[i];

printf "Relative Error: %.6f\n", sqrt(sum{i in 1..n} ((x[i] - 1)^2)) / sqrt(sum{i in 1..n} 1);

end;
