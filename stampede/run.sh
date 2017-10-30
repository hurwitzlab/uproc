#!/bin/bash

#SBATCH -J uproc
#SBATCH -N 1
#SBATCH -n 1
#SBATCH -p normal
#SBATCH -t 24:00:00
#SBATCH -A iPlant-Collabs

IN_DIR=""
OUT_DIR="$PWD/uproc-out"
SEQ_TYPE="dna"
IMG="uproc-1.2.0.img"
UPROC_DB_DIR="/work/05066/imicrobe/iplantc.org/data/uproc/dbnew"
UPROC_MODEL_DIR="/work/05066/imicrobe/iplantc.org/data/uproc/model"
PTHRESH=3
OTHRESH=2
LONG=0
SHORT=0
NUMERIC=0
PREDS=0
STATS=0
COUNTS=0

function lc() {
  wc -l "$1" | cut -d ' ' -f 1
}

function HELP() {
  printf "Usage:\n  %s -i IN_DIR \n\n" "$(basename "$0")"

  echo "Required arguments:"
  echo " -i iN_DIR (input directory)"
  echo ""
  echo "Optional arguments:"
  echo " -c COUNTS ($COUNTS)"
  echo " -d UPROC_DB_DIR ($UPROC_DB_DIR)"
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

if [[ $# -eq 0 ]]; then
  HELP
fi

while getopts :d:i:m:o:O:P:t:cfhlnps OPT; do
  case $OPT in
    c)
      COUNTS="1"
      ;;
    d)
      UPROC_DB_DIR="$OPTARG"
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

if [[ ! -d "$IN_DIR" ]]; then
    echo "IN_DIR \"$IN_DIR\" is not a directory"
    exit 1
fi

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

FILES=$(mktemp)
find "$IN_DIR" -type f > "$FILES"
NUM_FILES=$(lc "$FILES")

if [[ $NUM_FILES -lt 1 ]]; then
    echo "No files found in IN_DIR \"$IN_DIR\""
    exit 1
fi

echo "Will process NUM_FILES \"$NUM_FILES\""

OPTS="-P $PTHRESH -O $OTHRESH"
[[ $LONG -gt 0 ]]    && OPTS="$OPTS --long"
[[ $SHORT -gt 0 ]]   && OPTS="$OPTS --short"
[[ $NUMERIC -gt 0 ]] && OPTS="$OPTS --numeric"
[[ $STATS -gt 0 ]]   && OPTS="$OPTS --stats"
[[ $PREDS -gt 0 ]]   && OPTS="$OPTS --preds"
[[ $COUNTS -gt 0 ]]  && OPTS="$OPTS --counts"

PARAM="$$.param"
cat /dev/null > "$PARAM"

i=0
while read -r FILE; do
    BASENAME=$(basename "$FILE")
    let i++
    printf "%3d: %s\n" $i "$BASENAME"
    echo "singularity exec $IMG $PROG -o $OUT_DIR/$BASENAME $OPTS $UPROC_DB_DIR $UPROC_MODEL_DIR $FILE" >> "$PARAM"
    break
done < "$FILES"

NJOBS=$(lc "$PARAM")

if [[ $NJOBS -lt 1 ]]; then
    echo 'No launcher jobs to run!'
else
    export LAUNCHER_DIR="$HOME/src/launcher"
    export LAUNCHER_PLUGIN_DIR="$LAUNCHER_DIR/plugins"
    export LAUNCHER_WORKDIR="$PWD"
    export LAUNCHER_RMI="SLURM"
    export LAUNCHER_SCHED="interleaved"
    export LAUNCHER_JOB_FILE="$PARAM"
    [[ $NJOBS -gt 4  ]] && export LAUNCHER_PPN=4
    [[ $NJOBS -gt 16 ]] && export LAUNCHER_PPN=16
    echo "Started LAUNCHER $(date)"
    "$LAUNCHER_DIR/paramrun"
    echo "Ended LAUNCHER $(date)"
fi

echo "Done."
echo "Comments to kyclark@email.arizona.edu"
