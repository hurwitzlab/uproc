#!/bin/bash

#SBATCH -A iPlant-Collabs
#SBATCH -t 02:00:00
#SBATCH -N 1
#SBATCH -n 1
#SBATCH -J uprctest
#SBATCH -p development
#SBATCH --mail-type BEGIN,END,FAIL
#SBATCH --mail-user kyclark@email.arizona.edu

set -u

IN_DIR="$WORK/mouse/fasta"
OUT_DIR="$WORK/mouse/uproc-test"

./run.sh -i "$IN_DIR" -o "$OUT_DIR"
