# Autor: Bartosz Michalak
#### Zadanie 1 ####
# Funkcja monte_carlo_mse_bernoulli:
# Dla zadanych n, theta oraz nsim zwraca estymowane MSE dla:
#  S2_0 = (1/n) * sum (Xi - Xbar)^2  (wariancja ML z mianownikiem n)
#  S2   = (1/(n-1)) * sum (Xi - Xbar)^2 (estymator nieobciążony)
# oraz średnie z próbek i rozkładów (opcjonalnie).
monte_carlo_mse_bernoulli <- function(n, theta, nsim = 20000, seed = 12345) {
  set.seed(seed)
  true_var <- theta * (1 - theta)
  s2_0_vals <- numeric(nsim)
  s2_vals   <- numeric(nsim)
  for (i in seq_len(nsim)) {
    x <- rbinom(n, 1, theta)
    xbar <- mean(x)
    ssq <- sum((x - xbar)^2)
    s2_0_vals[i] <- ssq / n
    if (n > 1) {
      s2_vals[i] <- ssq / (n - 1)
    } else {
      s2_vals[i] <- NA
    }
  }
  mse_s2_0 <- mean((s2_0_vals - true_var)^2, na.rm = TRUE)
  mse_s2   <- mean((s2_vals - true_var)^2, na.rm = TRUE)
  bias_s2_0 <- mean(s2_0_vals, na.rm = TRUE) - true_var
  bias_s2   <- mean(s2_vals, na.rm = TRUE) - true_var
  list(n = n, theta = theta, true_var = true_var,
       mse_s2_0 = mse_s2_0, mse_s2 = mse_s2,
       bias_s2_0 = bias_s2_0, bias_s2 = bias_s2,
       var_estimates = c(var_s2_0 = var(s2_0_vals, na.rm=TRUE), var_s2 = var(s2_vals, na.rm=TRUE)))
}

# Przykładowe porównanie dla kilku (n,theta):
example_compare_bernoulli <- function() {
  params <- expand.grid(n = c(2, 5, 10, 30, 50), theta = c(0.1, 0.3, 0.5, 0.8))
  out <- apply(params, 1, function(row) {
    res <- monte_carlo_mse_bernoulli(n = as.integer(row["n"]), theta = as.numeric(row["theta"]), nsim = 20000)
    c(n = res$n, theta = res$theta,
      mse_s2_0 = res$mse_s2_0, mse_s2 = res$mse_s2,
      bias_s2_0 = res$bias_s2_0, bias_s2 = res$bias_s2,
      var_s2_0 = res$var_estimates["var_s2_0"], var_s2 = res$var_estimates["var_s2"])
  })
  out_df <- as.data.frame(t(out))
  # uporządkuj kolumny
  out_df$n <- as.integer(out_df$n)
  out_df$theta <- as.numeric(out_df$theta)
  out_df
}
