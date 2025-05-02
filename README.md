# COMP3522

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
