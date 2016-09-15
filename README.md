Data Assimilation
=================

Repository for Data-Assimilation study at AICS-DA Team.
This repository includes several contents, and will be separated.

Data Management
---------------
Data will be saved in two ways:

- Raw data: saved as a binary file (msgpack format) on local filesystem.
- Analysis (statistical) data: saved in elasticsearch (ES) server

Data format is shared in this project.

Components
-----------

Study for data-assimilation includes several components:

- Simulation (Lorenz 63 model, etc.)
  - Input Data: N/A
  - Output Data: Raw
  - Language: Rust, (Python)
- Data Assimilation (EnKF, LETKF, etc.)
  - Input Data: Raw
  - Output Data: Raw
  - Language: Rust, (Python)
- Data Analysis
  - Input Data: Raw
  - Output Data: Analysis
  - Language: Rust, Python (Jupyter)
- Visualization, browsing data
  - Input Data: Analysis
  - Output Data: N/A
  - Language: Python, JavaScript(d3js)

Development
------------

- Simulation
  - use ndarray-odeint (my private repository)

- Data Assimilation
  - Implement several algorithms:
    - [ ] Ensemble Kalman Filter (EnKF)
    - [ ] Localized Ensemble Transform Kalman Filter (LETKF)
    - [ ] Merging Particle Filter (MPF)

  - [ ] Split Assimilation part (w/o data I/O) as a public repository

- Data Analysis
  - Lyapunov Analysis
    - [ ] Covariant Lyapunov Vector (CLV)
    - [ ] Riccati equation

- Visualization
  - [ ] Data Browser (JavaScript)
