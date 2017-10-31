#!/bin/bash

echo "QUERY    \"${QUERY}\""
echo "SEQ_TYPE \"${SEQ_TYPE}\""
echo "COUNTS   \"${COUNTS}\""
echo "STATS    \"${STATS}\""
echo "LONG     \"${LONG}\""
echo "NUMERIC  \"${NUMERIC}\""
echo "OTHRESH  \"${OTHRESH}\""
echo "PTHRESH  \"${PTHRESH}\""
echo "SHORT    \"${SHORT}\""

sh run.sh ${QUERY} ${SEQ_TYPE} ${COUNTS} ${STATS} ${LONG} ${NUMERIC} ${OTHRESH} ${PTHRESH} ${SHORT}
