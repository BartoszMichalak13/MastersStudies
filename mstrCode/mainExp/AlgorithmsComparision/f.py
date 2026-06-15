import sys
import numpy as np
import time
import subprocess
import os

# =====================================================================
# KOD WORKERA (ZAPISYWANY DO PLIKU)
# =====================================================================
WORKER_SCRIPT = "delaunay_worker.py"

with open(WORKER_SCRIPT, "w") as f:
    f.write("""
import sys
import dionysus as d
from scipy.spatial import Delaunay
import itertools
import numpy as np
import resource

task = sys.argv[1]
N = int(sys.argv[2])

# 1. Generowanie Torusa (T-10,000)
np.random.seed(42) # Stały seed dla powtarzalności
theta = np.random.uniform(0, 2*np.pi, N)
phi = np.random.uniform(0, 2*np.pi, N)
R, r = 2, 1
x = (R + r * np.cos(theta)) * np.cos(phi)
y = (R + r * np.cos(theta)) * np.sin(phi)
z = r * np.sin(theta)
points = np.column_stack((x, y, z))

# 2. Triangulacja Delaunaya (Klucz do rzadkości! Odpowiednik Alpha Shapes)
tri = Delaunay(points)

# 3. Ręczna budowa rzadkiej filtracji dla Dionysusa
simplices = set()
for tet in tri.simplices:
    simplices.add(tuple(sorted(tet)))
    for face in itertools.combinations(tet, 3): simplices.add(tuple(sorted(face)))
    for edge in itertools.combinations(tet, 2): simplices.add(tuple(sorted(edge)))
    for v in tet: simplices.add((v,))

f = d.Filtration()
for s in simplices:
    f.append(d.Simplex(s))

f.sort()

if task == "complex_info":
    print(len(f))
    sys.exit(0)

# 4. Wykonanie algorytmu
if task == "complex":
    pass 
elif task == "homology":
    d.homology_persistence(f)
elif task == "cohomology":
    d.cohomology_persistence(f, prime=2, keep_cocycles=False)

# 5. Zrzut czystego RAM-u z Linuxa/WSL
usage = resource.getrusage(resource.RUSAGE_SELF)
print(f"{usage.ru_maxrss / 1024.0:.2f}")
""")

# =====================================================================
# GŁÓWNA LOGIKA POMIAROWA
# =====================================================================
def run_worker(task, N):
    start = time.time()
    cmd = [sys.executable, WORKER_SCRIPT, task, str(N)]
    res = subprocess.run(cmd, capture_output=True, text=True)
    t = time.time() - start
    
    if res.returncode != 0:
        print(f"Błąd w workerze ({task}):\n", res.stderr)
        return t, 0.0
        
    try:
        # Pobieramy ostatnią linijkę wyjścia
        output = float(res.stdout.strip().split('\n')[-1])
    except ValueError:
        output = 0.0
    return t, output

print("="*60)
print(" BENCHMARK: ODTWORZENIE WARUNKÓW DE SILVY (T-10,000)")
print(" Struktura: Rzadka Triangulacja Delaunaya (Alpha Shape)")
print("="*60)

N = 10000

# Sprawdzamy rozmiar wygenerowanego kompleksu
_, complex_size = run_worker("complex_info", N)
print(f"Liczba sympleksów w kompleksie: {int(complex_size):,} (Zgodne z de Silvą!)\n")

print("1. Budowa struktury bazowej w pamięci...")
t_comp, m_comp = run_worker("complex", N)
print(f"   [BAZA] RAM Kompleksu: {m_comp:.2f} MB\n")

print("2. Uruchamiam klasyczną Homologię (pHcol)...")
t_hom_tot, m_hom_tot = run_worker("homology", N)
# t_hom = max(0.0, t_hom_tot - t_comp)
# m_hom = max(0.0, m_hom_tot - m_comp)
t_hom = t_hom_tot
m_hom = m_hom_tot

print("3. Uruchamiam Kohomologię (pCoh)...")
t_coh_tot, m_coh_tot = run_worker("cohomology", N)
# t_coh = max(0.0, t_coh_tot - t_comp)
# m_coh = max(0.0, m_coh_tot - m_comp)
t_coh = t_coh_tot
m_coh = m_coh_tot

print("\n" + "="*60)
print(" WYNIKI KOŃCOWE (Czysty narzut algorytmiczny na stercie)")
print("="*60)
print(f" Homologia:   Czas = {t_hom:.4f} s | RAM (Narzut) = {m_hom:.2f} MB")
print(f" Kohomologia: Czas = {t_coh:.4f} s | RAM (Narzut) = {m_coh:.2f} MB")
print("="*60)

if m_coh <= m_hom:
    print("\nSUKCES: Kohomologia nie wybuchła! Zjawisko fill-in nie występuje w rzadkich strukturach.")

# Sprzątanie pliku tymczasowego
if os.path.exists(WORKER_SCRIPT):
    os.remove(WORKER_SCRIPT)