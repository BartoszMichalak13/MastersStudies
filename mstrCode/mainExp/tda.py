import numpy as np
import matplotlib.pyplot as plt

def plot_barcodes(dgms, ax, title, min_persistence=0.01):
    """Custom barcode plotting function with topological noise filtering."""
    colors = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd', '#8c564b']
    
    max_death = 0
    for dgm in dgms:
        if len(dgm) > 0:
            finite_deaths = dgm[dgm[:, 1] != np.inf][:, 1]
            if len(finite_deaths) > 0:
                max_death = max(max_death, np.max(finite_deaths))
                
    inf_end = max_death * 1.1 if max_death > 0 else 1.0
    current_y = 0
    yticks, ytick_labels = [], []
    
    for dim, dgm in enumerate(dgms):
        if len(dgm) == 0: continue
            
        persistence = np.where(np.isinf(dgm[:, 1]), inf_end * 2, dgm[:, 1]) - dgm[:, 0]
        valid_indices = np.where(persistence >= min_persistence)[0] if dim > 0 else np.arange(len(dgm))
        dgm_filtered = dgm[valid_indices]
        persistence_filtered = persistence[valid_indices]
        
        if len(dgm_filtered) == 0: continue
        
        sorted_indices = np.argsort(persistence_filtered)[::-1]
        sorted_dgm = dgm_filtered[sorted_indices]
        
        start_y = current_y
        for birth, death in sorted_dgm:
            is_inf = np.isinf(death)
            plot_death = inf_end if is_inf else death
            
            ax.hlines(y=current_y, xmin=birth, xmax=plot_death, colors=colors[dim % len(colors)], linewidth=2)
            if is_inf:
                ax.hlines(y=current_y, xmin=plot_death, xmax=inf_end * 1.05, colors=colors[dim % len(colors)], linewidth=2, linestyles='dashed')
            current_y += 1
            
        mid_y = (start_y + current_y - 1) / 2.0
        yticks.append(mid_y)
        ytick_labels.append(f"$H_{dim}$")
        current_y += max(3, int(len(dgm_filtered) * 0.1))
        
    ax.set_yticks(yticks)
    ax.set_yticklabels(ytick_labels)
    ax.set_xlabel("Filtration Parameter ($\epsilon$)")
    ax.set_title(title)
    ax.set_xlim(left=-0.05, right=inf_end * 1.05)
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.invert_yaxis()

def plot_landscapes(dgms, ax, title, target_dim=1, max_k=4):
    """Custom persistence landscape plotting function using lambda_k(t)."""
    colors = ['#d62728', '#2ca02c', '#bcbd22', '#1f77b4', '#ff7f0e']
    
    if target_dim >= len(dgms) or len(dgms[target_dim]) == 0:
        ax.set_title(f"{title} - Brak cech w H_{target_dim}")
        return
        
    dgm = dgms[target_dim]
    finite_dgm = dgm[dgm[:, 1] != np.inf]
    
    if len(finite_dgm) == 0:
        ax.set_title(f"{title} - Brak skończonych cech w H_{target_dim}")
        return

    min_birth, max_death = np.min(finite_dgm[:, 0]), np.max(finite_dgm[:, 1])
    t_vals = np.linspace(min_birth, max_death, 1000)
    
    b, d, t = finite_dgm[:, 0][:, None], finite_dgm[:, 1][:, None], t_vals[None, :]
    tents = np.maximum(0, np.minimum(t - b, d - t))
    tents_sorted = np.sort(tents, axis=0)[::-1, :]
    
    actual_k = min(max_k, len(finite_dgm))
    for k in range(actual_k):
        ax.plot(t_vals, tents_sorted[k, :], label=f'$\lambda_{{{k+1}}}$', color=colors[k % len(colors)], linewidth=2.5)
        ax.fill_between(t_vals, tents_sorted[k, :], alpha=0.15, color=colors[k % len(colors)])
        
    ax.set_title(f"{title} ($H_{target_dim}$)", fontsize=14)
    ax.set_xlabel("Filtration Parameter ($\epsilon$)", fontsize=12)
    ax.set_ylabel("Landscape Value", fontsize=12)
    ax.legend(loc='upper right')
    ax.grid(True, linestyle='--', alpha=0.6)
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)

def takens_embedding(series, delay, dimension):
    """
    Creates a Takens embedding (time-delay embedding) for a 1D time series.
    series: 1D numpy array of the time series data
    delay: the time delay (tau) -
    """
    N = len(series)
    if N - (dimension - 1) * delay <= 0:
        raise ValueError("Time series is too short for this dimension/delay.")

    embedded = np.array([series[i : i + (dimension - 1) * delay + 1 : delay]
                         for i in range(N - (dimension - 1) * delay)])
    return embedded
