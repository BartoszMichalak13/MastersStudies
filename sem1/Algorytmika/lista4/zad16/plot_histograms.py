import pandas as pd
import matplotlib.pyplot as plt

def plot_single_histogram(filename, title, color, output_file):
  df = pd.read_csv(filename)

  plt.figure(figsize=(8, 5))
  plt.bar(df['load'], df['count'], color=color, alpha=0.8)
  plt.title(title)
  plt.xlabel("Load (number of balls in bin)")
  plt.ylabel("Number of bins")
  plt.grid(True, linestyle="--", alpha=0.5)
  plt.tight_layout()
  plt.savefig(output_file)
  plt.show()

plot_single_histogram("uniform.csv", "Uniform Random Allocation", "skyblue", "uniform_hist.png")
plot_single_histogram("power2.csv", "Power of Two Choices", "orange", "power2_hist.png")

def plot_histogram(filename, title, color):
  df = pd.read_csv(filename)
  plt.bar(df['load'], df['count'], color=color, label=title, alpha=0.7)

plt.figure(figsize=(10, 5))

plot_histogram("uniform.csv", "Uniform Random Allocation", "skyblue")
plot_histogram("power2.csv", "Power of Two Choices", "orange")

plt.title("Histogram of Ball Allocations")
plt.xlabel("Load (number of balls in bin)")
plt.ylabel("Number of bins")
plt.legend()
plt.grid(True, linestyle="--", alpha=0.5)
plt.tight_layout()
plt.savefig("histogram_comparison.png")
plt.show()
