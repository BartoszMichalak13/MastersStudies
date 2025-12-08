# Autor: Bartosz Michalak
#### Zadanie 1 ####
# Funkcja monte_carlo_mse_bernoulli:
# Dla zadanych n, theta oraz nsim zwraca estymowane MSE dla:
#  S2_0 = (1/n) * sum (Xi - Xbar)^2
#  S2   = (1/(n-1)) * sum (Xi - Xbar)^2

library(ggplot2)
library(reshape2)

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
  list(n = n, theta = theta, true_var = true_var,
       mse_s2_0 = mse_s2_0, mse_s2 = mse_s2,
       var_estimates = c(var_s2_0 = var(s2_0_vals, na.rm=TRUE), var_s2 = var(s2_vals, na.rm=TRUE)))
}

example_compare_bernoulli_plot <- function() {
  # Define parameter grid
  params <- expand.grid(n = c(5, 10, 20, 50),
                        theta = seq(0.01, 0.99, by = 0.01))

  # Monte Carlo simulation
  out <- apply(params, 1, function(row) {
    res <- monte_carlo_mse_bernoulli(
      n = as.integer(row["n"]),
      theta = as.numeric(row["theta"]),
      nsim = 20000
    )
    c(n = res$n, theta = res$theta,
      mse_s2_0 = res$mse_s2_0, mse_s2 = res$mse_s2)
  })

  out_df <- as.data.frame(t(out))
  out_df$n <- as.integer(out_df$n)
  out_df$theta <- as.numeric(out_df$theta)

  # Convert to long format for ggplot
  plot_df <- melt(out_df,
                  id.vars = c("n", "theta"),
                  measure.vars = c("mse_s2_0", "mse_s2"),
                  variable.name = "Estimator",
                  value.name = "MSE")

  # Rename estimator labels
  plot_df$Estimator <- factor(plot_df$Estimator,
                              levels = c("mse_s2_0", "mse_s2"),
                              labels = c(expression(S[0]^2), expression(S^2)))

  # Create plot
  p <- ggplot(plot_df, aes(x = theta, y = MSE, color = Estimator)) +
    geom_line(size = 1) +
    facet_wrap(~ n, ncol = 2, labeller = label_bquote(n == .(n))) +
    theme_minimal(base_size = 14) +
    labs(
      title = "Porównanie MSE dla wybranych wartości n, θ",
      x = expression(theta),
      y = "MSE"
    ) +
    scale_color_manual(values = c("red", "blue"),
                       labels = c(expression(S[0]^2), expression(S^2))) +
    theme(
      legend.position = "bottom",
      legend.title = element_blank(),
      plot.title = element_text(hjust = 0.5)
    )

  print(p)
}
