# library(HDInterval)
# rozwiaz_zadanie_3_B10 <- function(n, alpha_prior, beta_prior, x, theta0, lambda) {
#   alpha_n <- alpha_prior + x; beta_n <- beta_prior + n - x

#   # Priors Odds (H1/H0)
#   pH0_prior <- pbeta(theta0, alpha_prior, beta_prior)
#   odds_prior <- (1 - pH0_prior) / pH0_prior

#   # 1. Exact
#   map_exact <- optimize(function(t) dbeta(t, alpha_n, beta_n), c(0,1), maximum=TRUE)$maximum
#   hpd_exact <- tryCatch(hdi(qbeta, 1-lambda, shape1=alpha_n, shape2=beta_n), error=function(e) c(NA, NA))
#   pH0_post <- pbeta(theta0, alpha_n, beta_n)
#   bf_exact <- ((1 - pH0_post)/pH0_post) / odds_prior

#   # 2. BCTG 1
#   mu <- alpha_n/(alpha_n+beta_n)
#   sd <- sqrt((alpha_n*beta_n)/((alpha_n+beta_n)^2*(alpha_n+beta_n+1)))
#   pH0_b1 <- pnorm(theta0, mu, sd)
#   bf_b1 <- ((1 - pH0_b1)/pH0_b1) / odds_prior

#   # 3. BCTG 2
#   mn <- (alpha_n-1)/(alpha_n+beta_n-2)
#   sdn <- sqrt(((alpha_n-1)*(beta_n-1))/(alpha_n+beta_n-2)^3)
#   pH0_b2 <- pnorm(theta0, mn, sdn)
#   bf_b2 <- ((1 - pH0_b2)/pH0_b2) / odds_prior

#   data.frame(
#     Metoda = c("Dokładna", "BCTG 1 (Momenty)", "BCTG 2 (Moda)"),
#     MAP = c(map_exact, mu, mn),
#     HPD_L = c(hpd_exact[1], mu - qnorm(1-lambda/2)*sd, mn - qnorm(1-lambda/2)*sdn),
#     HPD_R = c(hpd_exact[2], mu + qnorm(1-lambda/2)*sd, mn + qnorm(1-lambda/2)*sdn),
#     BayesFactor_B10 = c(bf_exact, bf_b1, bf_b2)
#   )
# }
# @