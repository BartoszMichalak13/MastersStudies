# Importing the Rlab library
library(Rlab)

#number of repeats
N <- 1000000

#probability of success
prob <- seq(0, 1, by = 0.01)

#generating bernoulli distribution
# xn <- dbern(0:1, prob)

# Function to calculate sample variance (s^2) and population variance (s0^2)
s0Square <- function(x) {

  n <- length(x)
  mean_x <- mean(x)

  return(sum((x - mean_x)^2) / (n - 1))
}

# Function to calculate mean square error (MSE)
sSquare <- function(x) {

  n <- length(x)
  mean_x <- mean(x)

  return(sum((x - mean_x)^2) / n)
}

# meanSquareError <- function(n, prob) {
#   # Generating Bernoulli distribution samples
#   samples <- rbern(N, prob)

#   # Splitting samples into groups of size n
#   sample_groups <- matrix(samples, nrow = N / n, ncol = n)

#   # Calculating MSE for each group
#   mse_values <- apply(sample_groups, 1, sSquare)

#   # Returning the average MSE across all groups
#   return(mean(mse_values))
# }

# Function that calculates MSE for S0 and S for different sample sizes
calculateMSEs <- function(sample_sizes, prob) {
  mse_s0_values <- sapply(sample_sizes, function(n) {
    samples <- rbern(N, prob)
    sample_groups <- matrix(samples, nrow = N / n, ncol = n)
    mse_s0_values <- apply(sample_groups, 1, s0Square)
    return(mean(mse_s0_values))
  })
  mse_s_values <- sapply(sample_sizes, function(n) {
    meanSquareError(n, prob)
    return(meanSquareError(n, prob))
  })

  # Returning the results as a data frame
  return(data.frame(S0 = mse_s0_values, S = mse_s_values))
}

#Compare MSE for S0 and S for different sample sizes and plot using calculateMSEs
mse_results <- calculateMSEs(c(5, 10, 20, 50), prob)
plot(mse_results$S0, type = "o", col = "blue", ylim = range(c(mse_results$S0, mse_results$S)), ylab = "MSE", xlab = "Sample Size")
lines(mse_results$S, type = "o", col = "red")
legend("topright", legend = c("S0", "S"), col = c("blue", "red"), lty = 1)

Error: unexpected end of input
