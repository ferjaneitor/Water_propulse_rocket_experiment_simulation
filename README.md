# Water Propulsion Rocket Simulation

Una simulación física completa de un cohete propulsado por agua, implementado en Rust con herramientas de visualización en Python.

## 📋 Descripción

Este proyecto simula el vuelo de un cohete de botella propulsado por agua y aire comprimido, modelando con precisión la dinámica de gases politrópicos, la mecánica de fluidos y la cinemática del proyectil. La simulación incluye:

- **Dinámica de gases politrópica**: Expansión adiabática del aire comprimido
- **Flujo de agua**: Ecuación de Bernoulli con coeficiente de descarga
- **Trayectoria balística**: Integración Runge-Kutta de 4º orden
- **Exportación de datos**: Resultados en formato CSV para análisis
- **Visualización**: Scripts Python para graficar trayectorias y series temporales

## 🚀 Características

### Motor de Simulación (Rust)
- Simulación de alta precisión con paso de tiempo configurable
- Modelo físico completo con conservación de energía
- Cálculo de empuje considerando descarga de agua y gas
- Seguimiento detallado de posición, velocidad, aceleración y masas
- Exportación de logs completos a CSV

### Herramientas de Visualización (Python)
- Detección automática de columnas de tiempo y posición
- Gráficas de trayectoria (Y vs X)
- Series temporales individuales y combinadas
- Conversión automática de tipos numéricos
- Soporte para submuestreo de datos grandes

## 🛠️ Requisitos

### Para la simulación (Rust)
- Rust 1.70+ (edición 2024)
- Cargo (incluido con Rust)

### Para la visualización (Python)
- Python 3.8+
- pandas
- matplotlib
- numpy

## 📦 Instalación

### Clonar el repositorio
```bash
git clone https://github.com/ferjaneitor/Water_propulse_rocket_experiment_simulation.git
cd physics_calculations
```

### Instalar dependencias de Python
```bash
pip install pandas matplotlib numpy
```

## 🎯 Uso

### Ejecutar la simulación

```bash
cargo run --release
```

Esto generará un archivo `logs.csv` con todos los datos de la simulación.

### Visualizar resultados

```bash
python csv_Vizualice.py logs.csv --out ./graficas
```

#### Opciones de visualización

```bash
# Especificar columnas manualmente
python csv_Vizualice.py logs.csv --time-col time_log --x-col x_log --y-col y_log

# Submuestrear datos (tomar cada N filas)
python csv_Vizualice.py logs.csv --step 10

# Excluir columnas de la gráfica combinada
python csv_Vizualice.py logs.csv --exclude-combined "x_log,y_log"

# Limitar número de filas leídas
python csv_Vizualice.py logs.csv --nrows 1000
```

## 📊 Parámetros de Simulación

Los parámetros físicos se encuentran en `src/constants.rs`:

```rust
// Parámetros del cohete
BOTTLE_VOLUME: 0.002 m³           // Volumen de la botella
NOZZLE_DIAMETER: 0.021 m          // Diámetro de la tobera
DRY_MASS: 0.05 kg                 // Masa en seco
DISCHARGE_COEFFICIENT: 0.8        // Coeficiente de descarga

// Condiciones iniciales
INITIAL_WATER_MASS: 1.0 kg        // Masa inicial de agua
INITIAL_AIR_PRESSURE_PSI: 90.0    // Presión manométrica inicial
LAUNCHING_ANGLE_DEG: 45.0         // Ángulo de lanzamiento

// Constantes físicas
GRAVITY: 9.81 m/s²
WATER_DENSITY: 1000.0 kg/m³
ADIABATIC_INDEX_AIR: 1.4
```

## 📁 Estructura del Proyecto

```
physics_calculations/
├── src/
│   ├── main.rs              # Punto de entrada
│   ├── simulation.rs        # Motor de simulación principal
│   ├── constants.rs         # Constantes físicas y parámetros
│   ├── math_utils/          # Utilidades matemáticas
│   │   ├── vector_2d.rs     # Implementación de vectores 2D
│   │   └── mod.rs
│   └── physics/             # Modelos físicos
│       ├── gas_dynamics.rs  # Dinámica de gases
│       └── mod.rs
├── csv_Vizualice.py         # Script de visualización
├── Cargo.toml               # Configuración de Rust
└── README.md
```

## 🔬 Modelo Físico

### Fase 1: Descarga de Agua
Durante esta fase, el agua es expulsada por la diferencia de presión entre el interior y el exterior:

- **Velocidad de salida**: Ecuación de Bernoulli con coeficiente de descarga
- **Flujo másico**: Basado en velocidad de salida y área de tobera
- **Presión interna**: Proceso politrópico (P·V^γ = constante)
- **Empuje**: F = ṁ·v_salida

### Fase 2: Descarga de Gas
Cuando el agua se agota, continúa la expulsión de aire:

- **Flujo de gas**: Modelo de flujo compresible
- **Expansión adiabática**: Relación presión-volumen
- **Empuje residual**: Decae conforme la presión se iguala

### Fase 3: Vuelo Balístico
Una vez que la presión interna iguala la atmosférica:

- **Trayectoria parabólica**: Solo actúa la gravedad
- **Sin resistencia del aire**: Modelo simplificado

## 📈 Datos de Salida

El archivo CSV generado incluye:

| Columna | Descripción | Unidad |
|---------|-------------|--------|
| `time_log` | Tiempo de simulación | s |
| `x_log` | Posición horizontal | m |
| `y_log` | Posición vertical | m |
| `x_velocity_log` | Velocidad horizontal | m/s |
| `y_velocity_log` | Velocidad vertical | m/s |
| `velocity_magnitur_log` | Magnitud de velocidad | m/s |
| `x_acceleration_log` | Aceleración horizontal | m/s² |
| `y_acceleration_log` | Aceleración vertical | m/s² |
| `acceleration_magnitur_log` | Magnitud de aceleración | m/s² |
| `water_mass_log` | Masa de agua | kg |
| `mass_log` | Masa total | kg |
| `water_volume_log` | Volumen de agua | m³ |
| `air_volume_log` | Volumen de aire | m³ |
| `pressure_log` | Presión interna | Pa |
| `thrust_force_log` | Fuerza de empuje | N |

## 🎓 Aplicaciones

Este simulador es útil para:

- **Educación**: Aprendizaje de física de fluidos y dinámica
- **Investigación**: Optimización de diseños de cohetes de agua
- **Competencias**: Predicción de trayectorias para competencias de cohetes
- **Experimentación**: Validación de modelos teóricos con experimentos reales

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor:

1. Haz fork del repositorio
2. Crea una rama para tu feature (`git checkout -b feature/mejora`)
3. Commit tus cambios (`git commit -am 'Añade nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/mejora`)
5. Abre un Pull Request

## 📝 Licencia

Este proyecto es de código abierto y está disponible bajo la licencia MIT.

## 👤 Autor

**ferjaneitor**

- GitHub: [@ferjaneitor](https://github.com/ferjaneitor)
- Repositorio: [Water_propulse_rocket_experiment_simulation](https://github.com/ferjaneitor/Water_propulse_rocket_experiment_simulation)

## 🙏 Agradecimientos

- Basado en modelos físicos clásicos de propulsión por agua
- Implementación inspirada en proyectos educativos de física computacional
- Comunidad de Rust por las excelentes herramientas de desarrollo

---

**¡Buena suerte con tus lanzamientos! 🚀💧**
