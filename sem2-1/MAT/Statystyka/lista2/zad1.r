bayesian_binomial_post_summary <- function(
  n,
  x,
  prior = c("Laplace", "Jeffreys", "MDIP")
) {

  if (!is.numeric(n) || n <= 0 || n != round(n) ||
    !is.numeric(x) || x < 0 || x > n || x != round(x)) {
    stop("n musi być dodatnią liczbą całkowitą, a x musi być całkowitą liczbą z zakresu [0, n].")
  }

  results <- data.frame(
    Prior = character(),
    Expected_Value = numeric(),
    Mode = numeric(),
    stringsAsFactors = FALSE
  )

  # (a) Laplace (Beta(1, 1))
  if ("Laplace" %in% prior) {
    alpha_L <- x + 1
    beta_L <- n - x + 1
    E_L <- alpha_L / (alpha_L + beta_L)
    Mode_L <- ifelse(
      x > 0 && n - x > 0, (alpha_L - 1) /
        (alpha_L + beta_L - 2), ifelse(x == 0, 0, 1)
    )
    results[nrow(results) + 1, ] <- list("Laplace", E_L, Mode_L)
  }

  # (b) Jeffreys (Beta(1/2, 1/2))
  if ("Jeffreys" %in% prior) {
    alpha_J <- x + 0.5
    beta_J <- n - x + 0.5
    E_J <- alpha_J / (alpha_J + beta_J)

    # Moda (gęstość dąży do nieskończoności na brzegu, gdy x=0 lub x=n)
    if (x == 0) {
      Mode_J <- 0
    } else if (x == n) {
      Mode_J <- 1
    } else {
      Mode_J <- (alpha_J - 1) / (alpha_J + beta_J - 2)
    }
    results[nrow(results) + 1, ] <- list("Jeffreys", E_J, Mode_J)
  }

  # (c) MDIP
  if ("MDIP" %in% prior) {
    dmdip.binom <- function(n, theta) {
      pmf <- outer(
        0:n,
        theta,
        Vectorize(\(x, theta) dbinom(x, size = n, prob = theta))
      )
      pmf[pmf == 0] <- 1e-15
      entropy <- colSums(pmf * log(pmf), na.rm = TRUE)
      exp(entropy)
    }

    E_M <- integrate(\(theta) theta * dbinom(x, n, theta) *
                       dmdip.binom(n, theta), 0, 1)$value /
                       integrate(\(theta) dbinom(x, n, theta) *
                       dmdip.binom(n, theta), 0, 1)$value

    Mode_M <- optimize(
      \(theta) log(dbinom(x, n, theta)) + log(dmdip.binom(n, theta)),
      interval = c(0, 1),
      maximum = TRUE
    )$maximum

    results[nrow(results) + 1, ] <- list("MDIP", E_M, Mode_M)
  }
  results$Expected_Value <- as.numeric(results$Expected_Value)
  results$Mode <- as.numeric(results$Mode)

  return(results)
}


n_example <- 1000
x_example <- 545

cat(sprintf("Wyniki dla n = %d i x = %d:\n", n_example, x_example))
example_results <- bayesian_binomial_post_summary(n_example, x_example, prior = c("Laplace", "Jeffreys", "MDIP"))
print(example_results)
