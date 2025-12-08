# Autor: Bartosz Michalak
library(ggplot2)

risk_entropy <- function(n, theta, a, b) {
  x <- 0:n
  pmf <- dbinom(x, size = n, prob = theta)
  d_x <- (x + a) / (n + a + b)
  eps <- .Machine$double.eps
  theta_clamped <- min(max(theta, eps), 1 - eps)
  term1 <- theta_clamped * log(theta_clamped / d_x)
  term2 <- (1 - theta_clamped) * log((1 - theta_clamped) / (1 - d_x))
  loss_x <- term1 + term2
  sum(pmf * loss_x)
}

risk_curve <- function(n, a, b, thetas = seq(0.001, 0.999, length.out = 301)) {
  sapply(thetas, function(th) risk_entropy(n, th, a, b))
}

#### Generate combined dataset for multiple n values ####
risk_data_multi <- function(n_values, thetas = seq(0.001, 0.999, length.out = 301)) {
  df_list <- list()

  for (n in n_values) {
    # estimator 1: a=b=1
    r1 <- risk_curve(n, a = 1, b = 1, thetas)
    # estimator 2: a=b=n/2
    r2 <- risk_curve(n, a = n/2, b = n/2, thetas)

    df_list[[length(df_list) + 1]] <- data.frame(
      n = factor(n),
      theta = rep(thetas, 2),
      risk = c(r1, r2),
      estimator = factor(rep(c("d[1,1]", "d[n/2,n/2]"), each = length(thetas)))
    )
  }
  do.call(rbind, df_list)
}

#### Plot with facets by n ####
plot_risk_multi <- function(n_values = c(5, 10, 20, 50)) {
  df <- risk_data_multi(n_values)

  ggplot(df, aes(x = theta, y = risk, color = estimator)) +
    geom_line(linewidth = 1) +
    facet_wrap(~ n, scales = "free_y") +
    theme_minimal(base_size = 14) +
    labs(
      title = "Porównanie funkcji ryzyka dla różnych n",
      x = expression(theta),
      y = expression(R(theta, d[a,b])),
      color = "Estymator"
    ) +
    scale_color_manual(values = c("#0072B2", "#E69F00"),
                       labels = c(expression(d[1,1]), expression(d[n/2,n/2]))) +
    theme(
      plot.title = element_text(hjust = 0.5, face = "bold"),
      legend.position = "top",
      strip.text = element_text(face = "bold")
    )
}