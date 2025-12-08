
# Wymagane pakiety:
# library(ggplot2); library(tidyr); library(dplyr); library(gridExtra)

# 1. Funkcja obliczająca parametry BCTG
aproksymacja_BCTG <- function(n, x, alfa, beta) {
  alfa_n <- alfa + x
  beta_n <- beta + n - x

  mu_n_v1 <- alfa_n / (alfa_n + beta_n)
  delta_n_sq_v1 <- (alfa_n * beta_n) /
    ((alfa_n + beta_n)^2 * (alfa_n + beta_n + 1))

  if (alfa_n > 1 && beta_n > 1) {
    m_n_v2 <- (alfa_n - 1) / (alfa_n + beta_n - 2)
    v_n_sq_v2 <- ((alfa_n - 1) * (beta_n - 1)) / (alfa_n + beta_n - 2)^3
  } else {
    m_n_v2 <- NA
    v_n_sq_v2 <- NA
  }

  return(list(
    mu1 = mu_n_v1,
    sd1 = sqrt(delta_n_sq_v1),
    mu2 = m_n_v2,
    sd2 = sqrt(v_n_sq_v2)
  ))
}

# 2. Funkcja generująca wykres
plot_bctg_comparison <- function(n, x, alfa = 0.5, beta = 0.5) {
  params <- aproksymacja_BCTG(n, x, alfa, beta)

  mu1 <- params$mu1[[1]]
  sigma1 <- params$sd1[[1]]
  mu2 <- params$mu2[[1]]
  sigma2 <- params$sd2[[1]]

  # Sprawdzenie, czy parametry są NA (ważne dla BCTG 2)
  if (is.na(mu1) || is.na(sigma1)) stop("Błąd: Parametry BCTG 1 są nieliczbowe (NA).")

  alfa_post <- alfa + x
  beta_post <- beta + n - x
  theta_range <- seq(0.001, 0.999, length.out = 500)

  plot_data <- data.frame(
    Theta = theta_range,
    Beta_Post = dbeta(theta_range, alfa_post, beta_post),
    BCTG_V1 = dnorm(theta_range, mean = mu1, sd = sigma1)
  )

  # Warunkowe dodawanie BCTG V2
  if (is.na(mu2) || is.na(sigma2)) {
      plot_data$BCTG_V2 <- 0
      label_v2 <- "BCTG 2 (Moda) - NA"
  } else {
      plot_data$BCTG_V2 <- dnorm(theta_range, mean = mu2, sd = sigma2)
      label_v2 <- sprintf("BCTG 2 (Moda) - N(%.3f, %.6f)", mu2, sigma2^2)
  }

  plot_long <- plot_data %>%
    pivot_longer(
      cols = -Theta, names_to = "Rozklad", values_to = "Gestosc"
    )

  plot_long$Rozklad <- factor(plot_long$Rozklad,
                              levels = c("Beta_Post", "BCTG_V1", "BCTG_V2"),
                              labels = c("A posteriori Beta (Jeffreys)",
                                         sprintf("BCTG 1 (Średnia) - N(%.3f, %.6f)", mu1, sigma1^2),
                                         label_v2))

  p <- ggplot(plot_long, aes(x = Theta, y = Gestosc, color = Rozklad)) +
    geom_line(linewidth = 1) +
    labs(
      title = paste("Aproksymacja BCTG (n=", n, ", x=", x, ")"),
      x = expression(theta), y = "Gęstość", color = "Rozkład"
    ) +
    scale_color_manual(values = c("blue", "red", "darkgreen")) +
    theme_minimal() +
    theme(legend.position = "bottom", plot.title = element_text(size = 10, hjust = 0.5))

  return(p)
}

# 3. Wykonanie i Połączenie Wykresów
N_values <- c(30, 50, 100)
X_values <- c(15, 30, 60)

plots <- list()
for (i in 1:length(N_values)) {
  plots[[i]] <- plot_bctg_comparison(N_values[i], X_values[i])
}

if (requireNamespace("gridExtra", quietly = TRUE)) {
    final_plot <- gridExtra::grid.arrange(grobs = plots, ncol = 3,
                           top = grid::textGrob("Porównanie gęstości rozkładu a posteriori i aproksymacji BCTG", gp=grid::gpar(fontsize=14, fontface="bold")))
    ggplot2::ggsave("bctg_combined_plots_R.png", plot = final_plot, width = 12, height = 5, units = "in", dpi = 300)
} else {
    message("Pakiet 'gridExtra' nie jest zainstalowany. Wykresy zostaną wyświetlone osobno.")
    for (p in plots) {
        print(p)
    }
}