# COMP3522

- `COMP3522_Final-main`
    - Final flow showcase
- `austin/data_generation`
    - contains pipeline to generate data from `main.csv`
    - requires rust and cargo
    - inside `austin/data_generation/`, run with
        1. `cargo run --release -- -m no_change`
        2. `cargo run --release -- -m add_columns`
        3. `cargo run --release -- -m change_once`
        4. `cargo run --release -- -m combine`
- `austin/bus analysis.ipynb`
    - contains some initial EDA on the generated data
- `austin/regression.ipynb`
    - contains multiple attempts at feedforward neural network, and cross
    validation between neural network and random forest.
    - The final models used and cross validations are in the last 2 sections of
    the notebook
- `kelvin/data`
    - contains minibus data and data for linear regression
- `kelvin/func_/analysis.py`
    - for EDA on minibus
- `kelvin/func_/linear_regression.ipynb`
    - contains training procedure of linear regression (OLS, WLS)
    - generates graphs
- `kelly/Data`
    - contains dataset for Model Training
    - include demographic, transport, and final dataset
- `kelly/MTR_Data`
    - contains full MTR data
- `kelly/DataCleaning`
    - `DataCleaning_Demographics.ipynb`: data filtering and merging demographic data
    - `DataCleaning_MTR.ipynb`: pipeline for collecting MTR data
- `kelly/EDA`
    - `EDA_Transport_data.ipynb`: EDA on all transport data
    - `EDA_MTR.ipynb`: EDA on MTR
- `kelly/Methdology`
    - contains the main methodology for project focus
    - `Stage1_Find_Expensive.ipynb`: Identifying expensive pairs and visualization
    - `Stage2_Find_Unreasonable.ipynb`: Identifying unreasonable pairs and visualisation
