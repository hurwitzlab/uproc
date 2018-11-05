#!/bin/bash

#SBATCH -J uproc
#SBATCH -N 1
#SBATCH -n 1
#SBATCH -p normal
#SBATCH -t 24:00:00
#SBATCH -A iPlant-Collabs

module load tacc-singularity

set -u

IMG="/work/05066/imicrobe/singularity/uproc-1.2.0-1.img"

if [[ ! -e "$IMG" ]]; then
    echo "Missing Singularity image \"$IMG\""
    exit 1
fi

UPROC_DATA="/work/05066/imicrobe/iplantc.org/data/uproc/"

singularity exec $IMG run_uproc "$@" -o "uproc-dna-out" -a "$UPROC_DATA/annotations" -d "$UPROC_DATA/db" -m "$UPROC_DATA/model"

echo "Comments to kyclark@email.arizona.edu"
