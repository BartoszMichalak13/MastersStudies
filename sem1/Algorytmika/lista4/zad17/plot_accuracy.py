import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv("approx_count_accuracy.csv")

plt.figure(figsize=(10, 5))
plt.plot(df['true_n'], df['relative_error'], label="Relative Error", color="teal")
# plt.axhline(0.1, color='red', linestyle='--', label="10% error")
plt.xlabel("True number of stations (n)")
plt.ylabel("Relative error")
plt.title("Accuracy of Approximate Counting Algorithm")
plt.grid(True, linestyle='--', alpha=0.5)
plt.legend()
plt.tight_layout()
plt.savefig("approx_count_accuracy.png")
plt.show()
