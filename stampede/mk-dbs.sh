#!/bin/bash

#SBATCH -J uprcimpt
#SBATCH -N 1
#SBATCH -n 1
#SBATCH -p normal
#SBATCH -t 24:00:00
#SBATCH -A iPlant-Collabs

set -u

IN_DIR="/work/05066/imicrobe/iplantc.org/data/uproc/dbs"
OUT_DIR="/work/05066/imicrobe/iplantc.org/data/uproc/dbnew"
IMG="uproc-1.2.0.img"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"
find "$OUT_DIR" -type f -exec rm {} \;

FILES=$(mktemp)
find "$IN_DIR" -type f > "$FILES"

while read -r FILE; do
    BASENAME=$(basename "$FILE")
    EXT=${BASENAME##*.}

    if [[ $EXT == 'gz' ]]; then
        echo "Unzipping $FILE"
        gunzip "$FILE"
        FILE=${FILE%.$EXT}
    fi

    CLEAN=$(basename "$BASENAME" '.uprocdb' | sed "s/_.*//")
    DB_DIR="$OUT_DIR/$CLEAN"

    [[ ! -d "$DB_DIR" ]] && mkdir -p "$DB_DIR"

    echo "$BASENAME => $DB_DIR"

    singularity exec $IMG uproc-import "$FILE" "$DB_DIR"
done < "$FILES"

echo "Done."
