#!/bin/bash

#SBATCH -A iPlant-Collabs
#SBATCH -p normal
#SBATCH -t 24:00:00
#SBATCH -N 1
#SBATCH -n 1
#SBATCH -J uprctest
#SBATCH --mail-type BEGIN,END,FAIL
#SBATCH --mail-user kyclark@email.arizona.edu

set -u

#IN_DIR="$WORK/data/pov/orfs"
IN_DIR="$WORK/data/pov/fasta"
OUT_DIR="$SCRATCH/uproc-out/mouse-dna"

./run.sh -i "$IN_DIR" -o "$OUT_DIR" -s -n -c
