# Autor: Bartosz Michalak
library(ggplot2)

#### Zadanie 2 ####
# Rozkład dwumianowy B(n,theta). Estymator:
# d_{a,b}(x) = (x + a) / (n + a + b)
# Funkcja strat (entropic / log loss) L(theta, d) = 
#   theta * log(theta / d) + (1-theta) * log((1-theta)/(1-d))
# (dla theta in {0,1} traktujemy granice odpowiednio).
# Funkcja risk_entropy(n, theta, a, b) liczy R(theta, d_{a,b}) = E_X[L(theta, d(X))].
risk_entropy <- function(n, theta, a, b) {
  # wektor x
  x <- 0:n
  pmf <- dbinom(x, size = n, prob = theta)
  d_x <- (x + a) / (n + a + b)
  # unikamy log(0) — ale dla a,b>0 d_x jest w (0,1)
  # obsługa theta=0 i theta=1 (limit)
  eps <- .Machine$double.eps
  theta_clamped <- min(max(theta, eps), 1 - eps)
  term1 <- if (theta_clamped == 0) 0 else theta_clamped * log(theta_clamped / d_x)
  term2 <- (1 - theta_clamped) * log((1 - theta_clamped) / (1 - d_x))
  loss_x <- term1 + term2
  sum(pmf * loss_x)
}

# Funkcja, która dla zadanych n, a, b zwraca wektor R(theta) na siatce theta (użyteczne do wykresów)
risk_curve <- function(n, a, b, thetas = seq(0, 1, length.out = 201)) {
  sapply(thetas, function(th) risk_entropy(n, th, a, b))
}

# Rysowanie wykresów dla zadanych n i par a,b
plot_risk_da_b <- function(n, thetas = seq(0.001, 0.999, length.out = 301)) {
  par(mfrow = c(1,2))
  # (a) a=1, b=1
  r1 <- risk_curve(n, a = 1, b = 1, thetas = thetas)
  plot(thetas, r1, type = "l", main = paste0("n=",n,", a=1, b=1"), ylab="R(θ, d)", xlab=expression(theta))
  # (b) a=n/2, b=n/2
  r2 <- risk_curve(n, a = n/2, b = n/2, thetas = thetas)
  plot(thetas, r2, type = "l", main = paste0("n=",n,", a=n/2, b=n/2"), ylab="R(θ, d)", xlab=expression(theta))
  par(mfrow = c(1,1))
}




#### Dodatkowe funkcje do rysowania wykresów z ggplot2 ####
# Funkcja do generowania danych dla funkcji ryzyka
risk_data <- function(n) {
  theta_vals <- seq(0, 1, length.out = 200)

  # R(theta, d_{1,1})
  a1 <- 1; b1 <- 1
  R1 <- sapply(theta_vals, function(theta) {
    theta * (1 - theta) / (n + a1 + b1)^2 +
      (theta * (n + a1) - (1 - theta) * b1)^2 / (n + a1 + b1)^2
  })

  # R(theta, d_{n/2, n/2})
  a2 <- n/2; b2 <- n/2
  R2 <- sapply(theta_vals, function(theta) {
    theta * (1 - theta) / (n + a2 + b2)^2 +
      (theta * (n + a2) - (1 - theta) * b2)^2 / (n + a2 + b2)^2
  })

  # zwracamy ramkę danych dla ggplot
  data.frame(
    theta = rep(theta_vals, 2),
    risk = c(R1, R2),
    estimator = factor(rep(c("d_{1,1}", "d_{n/2,n/2}"), each = length(theta_vals)))
  )
}

# Funkcja do rysowania wykresu
plot_risk_ggplot <- function(n) {
  df <- risk_data(n)

  ggplot(df, aes(x = theta, y = risk, color = estimator)) +
    geom_line(linewidth = 1.2) +
    theme_minimal(base_size = 14) +
    labs(
      title = paste("Porównanie funkcji ryzyka dla n =", n),
      x = expression(theta),
      y = expression(R(theta, d[a,b])),
      color = "Estymator"
    ) +
    scale_color_manual(values = c("#0072B2", "#E69F00")) +
    theme(
      plot.title = element_text(hjust = 0.5, face = "bold"),
      legend.position = "top"
    )

}




# Jeśli chcesz od razu uruchomić wszystkie przykłady:
# run_all_examples()

# Alternatywnie — przykłady krok po kroku (zalecane w sesji interaktywnej):
# 1) Zadanie 1: porównanie (tu pokazujemy tylko jedną linię)
# res1 <- monte_carlo_mse_bernoulli(n = 10, theta = 0.3, nsim = 20000); print(res1)
# 2) Zadanie 2: wykresy (np. dla n=10)
# plot_risk_da_b(n = 10)
# 3) Zadanie 3: optymalizacja a,b dla prior Be(1,1) i n=20
# opt_res <- optimize_ab_for_prior(n = 20); print(opt_res)

# Koniec skryptu
