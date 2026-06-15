import dionysus as d
import numpy as np

# 1. Wczytujemy punkty (zakładam, że masz już wygenerowany torus.csv dla N=500)
points = np.loadtxt("torus.csv", delimiter=",")

# 2. Budujemy kompleks (Tego nie chcemy wliczać do narzutu, 
#    ale musimy to zrobić, żeby mieć wejście dla kohomologii)
f = d.fill_rips(points, 3, 2.0)

# 3. Odpalamy kohomologię (To jest nasz główny podejrzany!)
#    Wyłączamy keep_cocycles, żeby zbadać czysty narzut roboczy
print("Rozpoczynam obliczenia kohomologii...")
d.cohomology_persistence(f, prime=2, keep_cocycles=False)
print("Gotowe.")