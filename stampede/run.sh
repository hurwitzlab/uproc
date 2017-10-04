#!/bin/bash

IN_DIR=""
OUT_DIR="$PWD/uproc-out"
SEQ_TYPE="dna"
IMG="uproc-1.2.0.img"
UPROC_DB_DIR="/work/05066/imicrobe/iplantc.org/data/uproc/dbs"
UPROC_MODEL_DIR="/work/05066/imicrobe/iplantc.org/data/uproc/model"

function lc() {
  wc -l "$1" | cut -d ' ' -f 1
}

function HELP() {
  printf "Usage:\n  %s -i IN_DIR \n\n" "$(basename "$0")"

  echo "Required arguments:"
  echo " -i iN_DIR (input directory)"
  echo ""
  echo "Optional arguments:"
  echo " -t SEQ_TYPE ($SEQ_TYPE)"
  exit 0
}

if [[ $# -eq 0 ]]; then
  HELP
fi

while getopts :i:o:t:h OPT; do
  case $OPT in
    i)
      IN_DIR="$OPTARG"
      ;;
    h)
      HELP
      ;;
    o)
      OUT_DIR="$OPTARG"
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
if [[ $SEQ_TYPE != "dna" ]]; then
    PROG="uproc-dna"
else if [[ $SEQ_TYPE != "prot" ]]; then
    PROG="uproc-prot"
else if[[ $SEQ_TYPE != "orf" ]]; then
    PROG="uproc-orf"
else
    echo "SEQ_TYPE \"$SEQ_TYPE\" must be dna, prot, or orf."
    exit 1
fi

FILES=$(mktemp)
find "$IN_DIR" -type > "$FILES"
NUM_FILES=$(lc "$FILES")

if [[ $NUM_FILES -lt 1 ]]; then
    echo "No files found in IN_DIR \"$IN_DIR\""
    exit 1
fi

i=0
while read -r FILE; do
    BASENAME=$(basename "$FILE")
    let i++
    printf "%3d: %s\n" $i "$BASENAME"
    singularity exec $IMG $PROG -o $OUT_DIR/$BASENAME $UPROC_DB_DIR $UPROG_MODEL_DIR $FILE
done < "$FILES"
