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


# Risk function under entropy (Kullback–Leibler) loss for Bernoulli model
risk_entropy <- function(n, theta, a, b) {
  # Distribution of number of successes (k) in n Bernoulli trials
  k_vals <- 0:n
  probs <- dbinom(k_vals, n, theta)

  # Decision rule: posterior mean of theta under Beta(a,b)
  d_vals <- (a + k_vals) / (a + b + n)

  # Entropy (KL) loss
  loss_vals <- theta * log(theta / d_vals) + (1 - theta) * log((1 - theta) / (1 - d_vals))

  # Expected risk over k
  sum(probs * loss_vals)
}

library(ggplot2)

# Function to plot expected risk surface for a,b > 0
plot_expected_risk_prior <- function(n, alpha = 1, beta = 1,
                                     a_seq = seq(0.1, 5, length.out = 40),
                                     b_seq = seq(0.1, 5, length.out = 40)) {
  grid <- expand.grid(a = a_seq, b = b_seq)

  # Compute expected risk for each (a,b)
  grid$risk <- sapply(1:nrow(grid), function(i) {
    expected_risk_prior(n, grid$a[i], grid$b[i], alpha, beta)
  })

  # Find optimal (a,b)
  opt <- optimize_ab_for_prior(n, alpha, beta)

  ggplot(grid, aes(x = a, y = b, fill = risk)) +
    geom_tile() +
    geom_point(aes(x = opt$par[1], y = opt$par[2]),
               color = "red", size = 3) +
    scale_fill_viridis_c(option = "plasma") +
    labs(title = bquote("Expected Bayes risk " ~ r(pi, d[a,b]) ~
                        " for " ~ n == .(n) ~ ", " ~ alpha == .(alpha) ~ ", " ~ beta == .(beta)),
         subtitle = bquote("Red point: optimal (" ~ a^"*" ~ "," ~ b^"*" ~ ")"),
         x = "a", y = "b", fill = "Expected risk") +
    theme_minimal(base_size = 14)
}

plot_expected_risk_prior_simple <- function(n, alpha = 1, beta = 1,
                                            a_seq = seq(0.1, 5, length.out = 30),
                                            b_seq = seq(0.1, 5, length.out = 30)) {
  grid <- expand.grid(a = a_seq, b = b_seq)

  # Oblicz ryzyko dla każdej pary (a,b)
  grid$risk <- sapply(1:nrow(grid), function(i) {
    expected_risk_prior(n, grid$a[i], grid$b[i], alpha, beta)
  })

  # Punkt optymalny
  opt <- optimize_ab_for_prior(n, alpha, beta)

  ggplot(grid, aes(x = a, y = b)) +
    geom_point(aes(color = risk), size = 2) +
    geom_point(aes(x = opt$par[1], y = opt$par[2]),
               color = "red", size = 4) +
    scale_color_viridis_c(option = "plasma") +
    labs(title = bquote("Expected Bayes risk " ~ r(pi, d[a,b]) ~
                        " for " ~ n == .(n) ~ ", " ~ alpha == .(alpha) ~ ", " ~ beta == .(beta)),
         subtitle = bquote("Red point: optimal (" ~ a^"*" ~ "," ~ b^"*" ~ ")"),
         x = "a", y = "b", color = "Expected risk") +
    theme_minimal(base_size = 14)
}
