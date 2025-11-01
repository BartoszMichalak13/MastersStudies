# Autor: Bartosz Michalak
#### Zadanie 3 ####
# r(pi, d_{a,b}) = E_{theta ~ Beta(alpha,beta)} [ R(theta, d_{a,b}) ]
# Zdefiniujemy funkcję expected_risk_prior(n, a, b, alpha, beta)
expected_risk_prior <- function(n, a, b, alpha = 1, beta = 1, rel.tol = 1e-8) {
  # integrand = R(theta, d)*dbeta(theta;alpha,beta)
  integrand <- function(theta) {
    sapply(theta, function(th) risk_entropy(n, th, a, b) * dbeta(th, shape1 = alpha, shape2 = beta))
  }
  res <- integrate(integrand, lower = 0, upper = 1, rel.tol = rel.tol, subdivisions = 200L)
  # wartość oczekiwana ryzyka
  res$value
}

# Funkcja do minimalizacji r(π, d_{a,b}) po a,b>0 (dla alpha=beta=1 domyślnie)
# użyjemy optymalizacji z ograniczeniami (L-BFGS-B)
optimize_ab_for_prior <- function(n, alpha = 1, beta = 1, a_start = 1, b_start = 1, lower = 1e-6, upper = 1000) {
  obj <- function(par) {
    a <- par[1]; b <- par[2]
    # penalizacja jeśli poza dopuszczalnymi granicami (powinna trzymać sie >0)
    if (a <= 0 || b <= 0) return(1e10)
    expected_risk_prior(n, a, b, alpha, beta)
  }
  res <- optim(par = c(a_start, b_start), fn = obj, method = "L-BFGS-B", lower = c(lower, lower), upper = c(upper, upper),
               control = list(trace = 0))
  list(par = res$par, value = res$value, convergence = res$convergence, message = res$message)
}