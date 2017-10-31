#!/bin/bash

#SBATCH -J uproc
#SBATCH -N 1
#SBATCH -n 1
#SBATCH -p normal
#SBATCH -t 24:00:00
#SBATCH -A iPlant-Collabs

set -u

IN_DIR=""
QUERY=""
OUT_DIR="$PWD/uproc-out"
SEQ_TYPE="dna"
IMG="uproc-1.2.0.img"
UPROC_DB_DIR_BASE="/work/05066/imicrobe/iplantc.org/data/uproc/db"
UPROC_MODEL_DIR="/work/05066/imicrobe/iplantc.org/data/uproc/model"
PTHRESH=3
OTHRESH=2
LONG=0
SHORT=0
NUMERIC=0
PREDS=0
STATS=0
COUNTS=0

export LAUNCHER_DIR="$HOME/src/launcher"
export LAUNCHER_PLUGIN_DIR="$LAUNCHER_DIR/plugins"
export LAUNCHER_WORKDIR="$PWD"
export LAUNCHER_RMI="SLURM"
export LAUNCHER_SCHED="interleaved"

function lc() {
    wc -l "$1" | cut -d ' ' -f 1
}

function HELP() {
    printf "Usage:\n  %s -i IN_DIR \n\n" "$(basename "$0")"
  
    echo "Required arguments:"
    echo " -i IN_DIR (input directory)"
    echo ""
    echo " OR"
    echo ""
    echo " -q QUERY (dirs/files)"
    echo ""
    echo "Optional arguments:"
    echo " -c COUNTS ($COUNTS)"
    echo " -d UPROC_DB_DIR_BASE ($UPROC_DB_DIR_BASE)"
    echo " -f STATS ($STATS)"
    echo " -l LONG ($LONG)"
    echo " -m UPROC_MODEL_DIR ($UPROC_MODEL_DIR)"
    echo " -n NUMERIC ($NUMERIC)"
    echo " -o OUT_DIR ($OUT_DIR)"
    echo " -O OTHRESH ($OTHRESH)"
    echo " -p PREDS ($PREDS)"
    echo " -P PTHRESH ($PTHRESH)"
    echo " -s SHORT ($SHORT)"
    echo " -t SEQ_TYPE ($SEQ_TYPE)"
    exit 0
}

[[ $# -eq 0 ]] && HELP

while getopts :d:i:m:o:O:P:q:t:cfhlnps OPT; do
    case $OPT in
      c)
          COUNTS="1"
          ;;
      d)
          UPROC_DB_DIR_BASE="$OPTARG"
          ;;
      f)
          STATS="1"
          ;;
      i)
          IN_DIR="$OPTARG"
          ;;
      h)
          HELP
          ;;
      l)
          LONG="1"
          ;;
      m)
          UPROC_MODEL_DIR="$OPTARG"
          ;;
      n)
          NUMERIC="1"
          ;;
      o)
          OUT_DIR="$OPTARG"
          ;;
      O)
          OTHRESH="$OPTARG"
          ;;
      p)
          PREDS="1"
          ;;
      P)
          PTHRESH="$OPTARG"
          ;;
      q)
          QUERY="$QUERY $OPTARG"
          ;;
      s)
          SHORT="1"
          ;;
      t)
          SEQ_TYPE="$OPTARG"
          ;;
      :)
          echo "Error: Option -$OPTARG requires an argument."
          exit 1
          ;;
      \?)
          echo "Error: Invalid option: -${OPTARG:-""}"
          exit 1
    esac
done

INPUT_FILES=$(mktemp)
if [[ -n "$IN_DIR" ]]; then
    if [[ -d "$IN_DIR" ]]; then
        find "$IN_DIR" -type f > "$INPUT_FILES"
    else
        echo "IN_DIR \"$IN_DIR\" is not a directory"
        exit 1
    fi
elif [[ -n "$QUERY" ]]; then
    for QRY in $QUERY; do
        if [[ -f "$QRY" ]]; then
            echo "$QRY" >> "$INPUT_FILES"
        elif [[ -d "$QRY" ]]; then
            find "$QRY" -type f -size +0c >> "$INPUT_FILES"
        else
            echo "\"$QRY\" is neither file nor directory"
        fi
    done
fi

NUM_FILES=$(lc "$INPUT_FILES")
if [[ $NUM_FILES -lt 1 ]]; then
    echo "Found no input files in QUERY/IN_DIR"
    exit 1
fi

echo "Will process NUM_FILES \"$NUM_FILES\""

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

PROG=""
if [[ $SEQ_TYPE == "dna" ]]; then
    PROG="uproc-dna"
elif [[ $SEQ_TYPE == "prot" ]]; then
    PROG="uproc-prot"
elif [[ $SEQ_TYPE == "orf" ]]; then
    PROG="uproc-orf"
else
    echo "SEQ_TYPE \"$SEQ_TYPE\" must be dna, prot, or orf."
    exit 1
fi

OPTS="-P $PTHRESH -O $OTHRESH"
[[ $LONG -gt 0 ]]    && OPTS="$OPTS --long"
[[ $SHORT -gt 0 ]]   && OPTS="$OPTS --short"
[[ $NUMERIC -gt 0 ]] && OPTS="$OPTS --numeric"
[[ $STATS -gt 0 ]]   && OPTS="$OPTS --stats"
[[ $PREDS -gt 0 ]]   && OPTS="$OPTS --preds"
[[ $COUNTS -gt 0 ]]  && OPTS="$OPTS --counts"

PARAM="$$.param"
cat /dev/null > "$PARAM"

UPROC_DB_DIRS=$(mktemp)
find "$UPROC_DB_DIR_BASE" -mindepth 1 -maxdepth 1 -type d > "$UPROC_DB_DIRS"
NUM_UPROC_DBS=$(lc "$UPROC_DB_DIRS")

if [[ $NUM_UPROC_DBS -lt 1 ]]; then
    echo "Cannot find any dirs in UPROC_DB_DIR_BASE \"$UPROC_DB_DIR_BASE\""
    exit 1
fi

i=0
while read -r FILE; do
    BASENAME=$(basename "$FILE")
    let i++
    printf "%6d: %s\n" $i "$BASENAME"
    #OUT_FILE=$(echo "$FILE" | perl -pe "s{$IN_DIR}{$OUT_DIR}")
    #BASEDIR=$(dirname "$OUT_FILE")
    #[[ ! -d "$BASEDIR" ]] && mkdir -p "$BASEDIR"

    while read -r DB_DIR; do
        DB_TYPE=$(basename "$DB_DIR") # e.g., kegg or pfam28
        OUT_FILE="$OUT_DIR/$DB_TYPE-$BASENAME"
        echo "singularity exec $IMG $PROG -o $OUT_FILE $OPTS $DB_DIR $UPROC_MODEL_DIR $FILE" >> "$PARAM"
    done < "$UPROC_DB_DIRS"
done < "$INPUT_FILES"

NJOBS=$(lc "$PARAM")

if [[ $NJOBS -lt 1 ]]; then
    echo 'No launcher jobs to run!'
else
    export LAUNCHER_JOB_FILE="$PARAM"
    [[ $NJOBS -ge 4  ]] && export LAUNCHER_PPN=4
    [[ $NJOBS -ge 16 ]] && export LAUNCHER_PPN=16
    echo "Started LAUNCHER $(date)"
    "$LAUNCHER_DIR/paramrun"
    echo "Ended LAUNCHER $(date)"
fi

echo "Done."
echo "Comments to kyclark@email.arizona.edu"
