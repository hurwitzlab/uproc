#!/bin/bash

#SBATCH -J uproc
#SBATCH -N 1
#SBATCH -n 1
#SBATCH -p normal
#SBATCH -t 24:00:00
#SBATCH -A iPlant-Collabs

module load tacc-singularity

IMG="/work/05066/imicrobe/singularity/uproc-1.2.0-1.img"

if [[ ! -e "$IMG" ]]; then
    echo "Missing Singularity image \"$IMG\""
    exit 1
fi

singularity exec $IMG run_uproc "$@"

echo "Comments to kyclark@email.arizona.edu"
