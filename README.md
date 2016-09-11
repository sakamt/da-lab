Data Assimilation
=================

Repository for Data-Assimilation study at AICS-DA Team.
This repository includes several contents, and will be separated.

Data Management
---------------
Data will be saved in two ways:

- Raw data: saved as a binary file (msgpack format) on local filesystem.
- Analysis (statistical) data: saved in elasticsearch server

Components
-----------

Study for data-assimilation includes several components:

- Simulation (Lorenz 63 model, etc.)
  - Input Data: N/A
  - Output Data: Raw
  - Language: Haskell, (Python)
- Data Assimilation (EnKF, LETKF, etc.)
  - Input Data: Raw
  - Output Data: Raw
  - Language: Haskell, (Python)
- Data Analysis
  - Input Data: Raw
  - Output Data: Analysis
  - Language: Haskell, Python (Jupyter)
- Visualization, browsing data
  - Input Data: Analysis
  - Output Data: N/A
  - Language: Python, JavaScript(d3js)

Development
------------

- Simulation
  - use repa-odeint (my public repository)
  - [ ] I/O for msgpack
- Data Assimilation
  - Implement several algorithms:
    - [ ] Ensemble Kalman Filter (EnKF)
    - [ ] Localized Ensemble Transform Kalman Filter (LETKF)
    - [ ] Merging Particle Filter (MPF)
  - [ ] I/O for msgpack
  - [ ] Split Assimilation part as a public repository
