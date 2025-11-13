from __future__ import annotations  # opcional, pero ayuda con tipos adelantados

# pyright: strict, reportUnknownMemberType=none

import os
from typing import Final

import matplotlib
matplotlib.use("Agg")  # backend sin interfaz gráfica

import matplotlib.pyplot as plt
from matplotlib.figure import Figure
from matplotlib.axes import Axes
import pandas as pd

# Ruta del CSV
CSV_PATH: Final[str] = r"C:\Visual Studio\Trabajo personal\physics_calculations\logs.csv"

# Carpeta donde se guardarán las imágenes
OUTPUT_DIR: Final[str] = r"C:\Visual Studio\Trabajo personal\salidas"

# ACTIVAR O NO la gráfica pesada con "todas las demás variables"
GENERAR_FIGURA_OTRAS: Final[bool] = False  # pon True si luego quieres probarla

# DPI BAJO PARA QUE GUARDE RÁPIDO
DPI_FIG: Final[int] = 80

# Máximo de puntos para las gráficas de grupos
MAX_POINTS_EXTRA: Final[int] = 5000


def downsample_df(df: pd.DataFrame, max_points: int = 5000) -> pd.DataFrame:
    """Devuelve un DataFrame submuestreado a un máximo de max_points filas."""
    n: int = len(df)
    if n <= max_points:
        return df
    step: int = max(n // max_points, 1)
    print(f"Submuestreando datos para grupos: tomando 1 de cada {step} filas.")
    return df.iloc[::step].reset_index(drop=True)


def main() -> None:
    # Asegura que la carpeta de salida exista
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    print("Carpeta actual:", os.getcwd())
    print("Leyendo archivo:", CSV_PATH)

    df: pd.DataFrame = pd.read_csv(CSV_PATH)

    print("\nColumnas encontradas:")
    print(df.columns.tolist())
    print(f"Número total de filas: {len(df)}")

    print("\nPrimeras filas:")
    print(df.head())

    # ======================================
    # 1) GRÁFICAS CON TODOS LOS PUNTOS
    #    (trayectoria y posición vs tiempo)
    # ======================================
    t_full = df["time_s"]
    x_full = df["x_m"]
    y_full = df["y_m"]

    # --- Trayectoria Y vs X (full) ---
    fig1: Figure
    ax1: Axes
    fig1, ax1 = plt.subplots(figsize=(6, 4))
    ax1.plot(x_full, y_full, rasterized=True, antialiased=False)
    ax1.set_xlabel("Posición X (m)")
    ax1.set_ylabel("Posición Y (m)")
    ax1.set_title("Trayectoria (Y vs X)")
    ax1.grid(True)
    ax1.set_aspect("equal", adjustable="box")

    out1 = os.path.join(OUTPUT_DIR, "trayectoria_y_vs_x.png")
    fig1.savefig(out1, dpi=DPI_FIG)
    print(f"Gráfica 1 guardada en: {out1}")
    plt.close(fig1)

    # --- X e Y vs tiempo (full) ---
    fig2: Figure
    ax2: Axes
    fig2, ax2 = plt.subplots(figsize=(6, 4))
    ax2.plot(t_full, x_full, label="x_m", rasterized=True, antialiased=False)
    ax2.plot(t_full, y_full, label="y_m", rasterized=True, antialiased=False)
    ax2.set_xlabel("Tiempo (s)")
    ax2.set_ylabel("Posición (m)")
    ax2.set_title("Posición X y Y vs Tiempo")
    ax2.legend(loc="upper right", fontsize=8)
    ax2.grid(True)

    out2 = os.path.join(OUTPUT_DIR, "posicion_xy_vs_tiempo.png")
    fig2.savefig(out2, dpi=DPI_FIG)
    print(f"Gráfica 2 guardada en: {out2}")
    plt.close(fig2)

    # ======================================
    # 2) GRÁFICAS DE GRUPOS (CON SUBMUESTREO)
    # ======================================
    df_extra: pd.DataFrame = downsample_df(df, MAX_POINTS_EXTRA)
    print(f"Filas usadas para grupos: {len(df_extra)}")

    t = df_extra["time_s"]

    # --- Figura global otras variables (opcional) ---
    if GENERAR_FIGURA_OTRAS:
        other_cols = [c for c in df_extra.columns if c not in ["time_s", "x_m", "y_m"]]
        print("\nGenerando figura con todas las variables excepto posición...")
        print("Columnas que se van a incluir:", other_cols)

        fig3: Figure
        ax3: Axes
        fig3, ax3 = plt.subplots(figsize=(8, 5))
        for c in other_cols:
            ax3.plot(t, df_extra[c], label=c, rasterized=True, antialiased=False)

        ax3.set_xlabel("Tiempo (s)")
        ax3.set_ylabel("Valor")
        ax3.set_title("Evolución de todas las variables excepto posición")
        ax3.legend(loc="upper right", fontsize=7)
        ax3.grid(True)

        out3 = os.path.join(OUTPUT_DIR, "variables_vs_tiempo.png")
        fig3.savefig(out3, dpi=DPI_FIG)
        print(f"Gráfica 3 guardada en: {out3}")
        plt.close(fig3)
    else:
        print("\nSaltando la figura global de 'todas las demás variables' (GENERAR_FIGURA_OTRAS = False).")

    # -------- Velocidades --------
    vel_cols = ["vx_m_s", "vy_m_s", "v_mag_m_s"]
    vel_cols_presentes = [c for c in vel_cols if c in df_extra.columns]

    if vel_cols_presentes:
        fig_v: Figure
        ax_v: Axes
        fig_v, ax_v = plt.subplots(figsize=(6, 4))
        for c in vel_cols_presentes:
            ax_v.plot(t, df_extra[c], label=c, rasterized=True, antialiased=False)
        ax_v.set_xlabel("Tiempo (s)")
        ax_v.set_ylabel("Velocidad (m/s)")
        ax_v.set_title("Velocidades vs Tiempo")
        ax_v.legend(loc="upper right", fontsize=8)
        ax_v.grid(True)

        out_v = os.path.join(OUTPUT_DIR, "velocidades_vs_tiempo.png")
        fig_v.savefig(out_v, dpi=DPI_FIG)
        print(f"Gráfica de velocidades guardada en: {out_v}")
        plt.close(fig_v)
    else:
        print("No se encontraron columnas de velocidad para graficar.")

    # -------- Aceleraciones --------
    acc_cols = ["ax_m_s2", "ay_m_s2", "a_mag_m_s2"]
    acc_cols_presentes = [c for c in acc_cols if c in df_extra.columns]

    if acc_cols_presentes:
        fig_a: Figure
        ax_a: Axes
        fig_a, ax_a = plt.subplots(figsize=(6, 4))
        for c in acc_cols_presentes:
            ax_a.plot(t, df_extra[c], label=c, rasterized=True, antialiased=False)
        ax_a.set_xlabel("Tiempo (s)")
        ax_a.set_ylabel("Aceleración (m/s²)")
        ax_a.set_title("Aceleraciones vs Tiempo")
        ax_a.legend(loc="upper right", fontsize=8)
        ax_a.grid(True)

        out_a = os.path.join(OUTPUT_DIR, "aceleraciones_vs_tiempo.png")
        fig_a.savefig(out_a, dpi=DPI_FIG)
        print(f"Gráfica de aceleraciones guardada en: {out_a}")
        plt.close(fig_a)
    else:
        print("No se encontraron columnas de aceleración para graficar.")

    # -------- Masas --------
    mass_cols = ["water_mass_kg", "total_mass_kg"]
    mass_cols_presentes = [c for c in mass_cols if c in df_extra.columns]

    if mass_cols_presentes:
        fig_m: Figure
        ax_m: Axes
        fig_m, ax_m = plt.subplots(figsize=(6, 4))
        for c in mass_cols_presentes:
            ax_m.plot(t, df_extra[c], label=c, rasterized=True, antialiased=False)
        ax_m.set_xlabel("Tiempo (s)")
        ax_m.set_ylabel("Masa (kg)")
        ax_m.set_title("Masas vs Tiempo")
        ax_m.legend(loc="upper right", fontsize=8)
        ax_m.grid(True)

        out_m = os.path.join(OUTPUT_DIR, "masas_vs_tiempo.png")
        fig_m.savefig(out_m, dpi=DPI_FIG)
        print(f"Gráfica de masas guardada en: {out_m}")
        plt.close(fig_m)
    else:
        print("No se encontraron columnas de masa para graficar.")

    # -------- Empuje (thrust_N) --------
    if "thrust_N" in df_extra.columns:
        fig_thr: Figure
        ax_thr: Axes
        fig_thr, ax_thr = plt.subplots(figsize=(6, 4))
        ax_thr.plot(t, df_extra["thrust_N"], label="thrust_N", rasterized=True, antialiased=False)
        ax_thr.set_xlabel("Tiempo (s)")
        ax_thr.set_ylabel("Empuje (N)")
        ax_thr.set_title("Empuje vs Tiempo")
        ax_thr.legend(loc="upper right", fontsize=8)
        ax_thr.grid(True)

        out_thr = os.path.join(OUTPUT_DIR, "thrust_vs_tiempo.png")
        fig_thr.savefig(out_thr, dpi=DPI_FIG)
        print(f"Gráfica de empuje guardada en: {out_thr}")
        plt.close(fig_thr)
    else:
        print("Columna 'thrust_N' no encontrada.")

    # -------- Presión absoluta (pressure_abs_Pa) --------
    if "pressure_abs_Pa" in df_extra.columns:
        fig_pre: Figure
        ax_pre: Axes
        fig_pre, ax_pre = plt.subplots(figsize=(6, 4))
        ax_pre.plot(t, df_extra["pressure_abs_Pa"], label="pressure_abs_Pa",
                    rasterized=True, antialiased=False)
        ax_pre.set_xlabel("Tiempo (s)")
        ax_pre.set_ylabel("Presión absoluta (Pa)")
        ax_pre.set_title("Presión Absoluta vs Tiempo")
        ax_pre.legend(loc="upper right", fontsize=8)
        ax_pre.grid(True)

        out_pre = os.path.join(OUTPUT_DIR, "presion_vs_tiempo.png")
        fig_pre.savefig(out_pre, dpi=DPI_FIG)
        print(f"Gráfica de presión guardada en: {out_pre}")
        plt.close(fig_pre)
    else:
        print("Columna 'pressure_abs_Pa' no encontrada.")


if __name__ == "__main__":
    main()