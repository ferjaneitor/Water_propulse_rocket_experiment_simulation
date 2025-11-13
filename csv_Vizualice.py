import os
import pandas as pd
import matplotlib.pyplot as plt

# ================================
# CONFIGURACIÓN
# ================================

# Ruta completa a tu archivo CSV
CSV_PATH = r"C:\Visual Studio\Trabajo personal\physics_calculations\logs.csv"

def main():
    # Mostrar desde dónde se está ejecutando el script
    print("Carpeta actual de ejecución:", os.getcwd())
    print("Intentando leer el archivo:", CSV_PATH)

    # ================================
    # 1. LEER CSV
    # ================================
    df = pd.read_csv(CSV_PATH)

    print("\nColumnas encontradas en el CSV:")
    print(df.columns.tolist())

    # Mostramos las primeras filas para verificar
    print("\nPrimeras filas del archivo:")
    print(df.head())

    # Verificamos que estén las columnas clave
    required_cols = ["time_s", "x_m", "y_m"]
    for col in required_cols:
        if col not in df.columns:
            raise ValueError(f"Falta la columna '{col}' en el CSV")

    # Asignamos variables principales
    t = df["time_s"]
    x = df["x_m"]
    y = df["y_m"]

    # ================================
    # 2. TRAYECTORIA Y vs X
    # ================================
    plt.figure(figsize=(8, 6))
    plt.plot(x, y)
    plt.xlabel("Posición X (m)")
    plt.ylabel("Posición Y (m)")
    plt.title("Trayectoria (Y vs X)")
    plt.grid(True)
    plt.axis("equal")  # Escala igual en X y Y para comparar bien
    plt.tight_layout()

    # ================================
    # 3. X Y Y vs TIEMPO
    # ================================
    plt.figure(figsize=(8, 6))
    plt.plot(t, x, label="x_m")
    plt.plot(t, y, label="y_m")
    plt.xlabel("Tiempo (s)")
    plt.ylabel("Posición (m)")
    plt.title("Posición X y Y vs Tiempo")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    # ================================
    # 4. TODAS LAS DEMÁS VARIABLES vs TIEMPO
    #    (excepto time_s, x_m, y_m)
    # ================================
    other_cols = [c for c in df.columns if c not in ["time_s", "x_m", "y_m"]]

    if other_cols:
        plt.figure(figsize=(10, 7))
        for c in other_cols:
            plt.plot(t, df[c], label=c)

        plt.xlabel("Tiempo (s)")
        plt.ylabel("Valor")
        plt.title("Evolución de todas las variables (excepto posición) vs Tiempo")
        plt.legend(loc="best")
        plt.grid(True)
        plt.tight_layout()
    else:
        print("No hay columnas adicionales distintas de time_s, x_m, y_m para graficar.")

    # ================================
    # 5. MOSTRAR TODAS LAS GRÁFICAS
    # ================================
    plt.show()


if __name__ == "__main__":
    main()
