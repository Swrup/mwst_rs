# MWST_rs

Compute the Minimum Weight Spanning Tree of the list of cities read from stdin.

It use Prim's algorithm and Rayon for parallelization.

Usage example :

    cargo build --release ; shuf -n 1000 citiesList.csv | ./target/release/mwst_rs ; python visualisation.py
        
