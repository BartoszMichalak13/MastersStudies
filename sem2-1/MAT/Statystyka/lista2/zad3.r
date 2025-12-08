rozwiaz_zadanie_3_zgodnie_z_definicja <- function(n, alpha_prior, beta_prior, x, theta0, lambda) {
  alpha_n <- alpha_prior + x
  beta_n <- beta_prior + n - x

  # ==============================================================================
  # 1. ROZKŁAD DOKŁADNY (Podejście numeryczne)
  # ==============================================================================
  # (a) ESTYMATOR MAP (Maximum A Posteriori)
  # Definicja 1: arg max h(theta | x)
  # Definiujemy funkcję gęstości i szukamy jej maksimum.
  funkcja_gestosci_a_posteriori <- function(theta) {
    dbeta(theta, shape1 = alpha_n, shape2 = beta_n)
  }

  # optimize() szuka maksimum w przedziale (0, 1)
  wynik_optymalizacji <- optimize(f = funkcja_gestosci_a_posteriori,
                                  interval = c(0, 1),
                                  maximum = TRUE)

  map_exact <- wynik_optymalizacji$maximum

  # (b) OBSZAR NAJWIĘKSZEJ GĘSTOŚCI (HPD)
  # Szukamy przedziału, gdzie gęstość na krańcach jest równa (dbeta(L) == dbeta(U))
  calc_hpd_custom <- function(a, b, conf_level) {
    obj_func <- function(p) {
      lower_q <- qbeta(p, a, b)
      upper_q <- qbeta(p + conf_level, a, b)
      return(dbeta(lower_q, a, b) - dbeta(upper_q, a, b))
    }
    # uniroot szuka p takiego, że różnica gęstości wynosi 0
    res <- uniroot(obj_func, interval = c(1e-10, 1 - conf_level - 1e-10))
    p_best <- res$root
    return(c(qbeta(p_best, a, b), qbeta(p_best + conf_level, a, b)))
  }

  hpd_exact <- calc_hpd_custom(alpha_n, beta_n, 1 - lambda)

  # (c) CZYNNIK BAYESA (Bayes Factor)
  # Hipoteza H0: theta <= theta0 (Całka z gęstości od 0 do theta0)
  # Hipoteza H1: theta > theta0  (Całka z gęstości od theta0 do 1)
  prob_H0_prior <- pbeta(theta0, alpha_prior, beta_prior)
  odds_prior <- prob_H0_prior / (1 - prob_H0_prior)

  prob_H0_post <- pbeta(theta0, alpha_n, beta_n)
  odds_post <- prob_H0_post / (1 - prob_H0_post)

  bf_exact <- odds_prior / odds_post

  # ==============================================================================
  # 2. APROKSYMACJE BCTG 1 (wartość oczekiwana)
  # ==============================================================================
  mu_n <- alpha_n / (alpha_n + beta_n)
  d_n_sq <- (alpha_n * beta_n) / ((alpha_n + beta_n)^2 * (alpha_n + beta_n + 1))
  sd_n <- sqrt(d_n_sq)

  map_bctg1 <- mu_n
  z_crit <- qnorm(1 - lambda / 2)
  hpd_bctg1 <- c(mu_n - z_crit * sd_n, mu_n + z_crit * sd_n)
  prob_H0_bctg1 <- pnorm(theta0, mean = mu_n, sd = sd_n)
  bf_bctg1 <- odds_prior / (prob_H0_bctg1 / (1 - prob_H0_bctg1))

  # ==============================================================================
  # 3. APROKSYMACJE BCTG 2 (moda)
  # ==============================================================================
  m_n <- (alpha_n - 1) / (alpha_n + beta_n - 2)
  v_n_sq <- ((alpha_n - 1) * (beta_n - 1)) / ((alpha_n + beta_n - 2)^3)
  sd_v <- sqrt(v_n_sq)
  map_bctg2 <- m_n
  hpd_bctg2 <- c(m_n - z_crit * sd_v, m_n + z_crit * sd_v)
  prob_H0_bctg2 <- pnorm(theta0, mean = m_n, sd = sd_v)
  bf_bctg2 <- odds_prior / (prob_H0_bctg2 / (1 - prob_H0_bctg2))

  results <- data.frame(
    Metoda = c("Dokładna", "BCTG 1", "BCTG 2"),
    MAP_Estymator = c(map_exact, map_bctg1, map_bctg2),
    HPD_Dol = c(hpd_exact[1], hpd_bctg1[1], hpd_bctg2[1]),
    HPD_Gora = c(hpd_exact[2], hpd_bctg1[2], hpd_bctg2[2]),
    BayesFactor_10 = c(bf_exact, bf_bctg1, bf_bctg2)
  )

  print(results)
  return(invisible(results))
}

# --- PRZYKŁADOWE WYWOŁANIE ---
# n, x: dane z próby
# alpha, beta: parametry a priori
# theta0: granica hipotezy (nie prawdziwa wartość!)
# lambda: poziom istotności
out <- rozwiaz_zadanie_3_zgodnie_z_definicja(n = 10, alpha_prior = 0.5, beta_prior = 0.5,
                                             x = 4, theta0 = 0.5, lambda = 0.05)