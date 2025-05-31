# # plot.py
# import pandas as pd
# import matplotlib.pyplot as plt
# import seaborn as sns

# df = pd.read_csv("output.csv")

# # Średni współczynnik konkurencyjności
# summary = df.groupby(["algorithm", "distribution"])["competitive_ratio"].mean().reset_index()

# plt.figure(figsize=(10, 6))
# sns.barplot(data=summary, x="distribution", y="competitive_ratio", hue="algorithm")
# plt.title("Średni współczynnik konkurencyjności")
# plt.ylabel("Średni współczynnik konkurencyjności")
# plt.xlabel("Rozkład")
# plt.legend(title="Algorytm")
# plt.tight_layout()
# plt.savefig("competitive_ratios.png")
# plt.show()

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# Wczytaj dane
df = pd.read_csv("output.csv")

# # Średni współczynnik konkurencyjności
# summary_ratio = df.groupby(["algorithm", "distribution"])["competitive_ratio"].mean().reset_index()

# plt.figure(figsize=(10, 6))
# sns.barplot(data=summary_ratio, x="distribution", y="competitive_ratio", hue="algorithm")
# plt.title("Średni współczynnik konkurencyjności")
# plt.ylabel("Średni współczynnik konkurencyjności")
# plt.xlabel("Rozkład")
# plt.legend(title="Algorytm")
# plt.tight_layout()
# plt.savefig("competitive_ratios.png")
# plt.show()

# Średnia liczba użytych kubełków
summary_bins = df.groupby(["distribution", "algorithm"])["bins_used"].mean().reset_index()
print( df.groupby(["distribution", "algorithm"])["bins_used"])
# summary_bins = df.groupby(["algorithm", "distribution"])["bins_used"].mean().reset_index()

plt.figure(figsize=(10, 6))
# sns.barplot(data=summary_bins, x="distribution", y="bins_used", hue="algorithm")
sns.barplot(data=summary_bins, x="algorithm", y="bins_used", hue="distribution")
plt.title("Średnia liczba użytych kubełków")
plt.ylabel("Średnia liczba kubełków")
plt.xlabel("Rozkład")
plt.legend(title="Algorytm")
plt.tight_layout()
plt.savefig("bins_used.png")
plt.show()
