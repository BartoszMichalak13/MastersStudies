import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

import matplotlib
matplotlib.use('Agg')
# Read the data
df = pd.read_csv("results.csv")
# Read the data
df = pd.read_csv("results.csv")

# Check column names
print("CSV Columns:", df.columns)

# Ensure column names are correct
expected_columns = {"n", "Strategy", "AvgCost", "Distribution"}
if not expected_columns.issubset(set(df.columns)):
    raise ValueError(f"CSV is missing expected columns: {expected_columns - set(df.columns)}")

# Create a PDF file to store all plots
pdf_filename = "access_cost_plots.pdf"
with PdfPages(pdf_filename) as pdf:
    # Get unique distributions
    distributions = df["Distribution"].unique()

    for dist in distributions:
        plt.figure(figsize=(10, 6))

        for strategy in df["Strategy"].unique():
            subset = df[(df["Distribution"] == dist) & (df["Strategy"] == strategy)]
            plt.plot(subset["n"], subset["AvgCost"], marker="o", label=strategy)

        plt.xscale("log")  # Log scale for better visualization
        plt.xlabel("Number of Operations (n)")
        plt.ylabel("Average Access Cost")
        plt.title(f"Access Cost for {dist} Distribution")
        plt.legend()
        plt.grid(True)

        # Save the current figure as a PNG file
        png_filename = f"{dist}_plot.png"
        plt.savefig(png_filename)  # Save as PNG file
        print(f"Saved PNG plot: {png_filename}")

        # Save the current figure to the PDF
        pdf.savefig()
        plt.close()

print(f"All plots saved to {pdf_filename}")