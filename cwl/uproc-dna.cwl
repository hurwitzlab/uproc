#!/usr/bin/env cwl-runner

cwlVersion: v1.0
class: CommandLineTool
hints:
  DockerRequirement:
    dockerPull: hurwitzlab/uproc:1.2.0

baseCommand: run_uproc

inputs:
  query:
    type: File
    inputBinding:
      position: 1
      prefix: --query
  uproc_db_dir:
    type: Directory
    inputBinding:
      position: 1
      prefix: --uproc_db_dir
  uproc_model_dir:
    type: Directory
    inputBinding:
      position: 1
      prefix: --uproc_model_dir
  annotation_dir:
    type: Directory
    inputBinding:
      position: 1
      prefix: --annotation_dir
  othresh:
    type: string?
    inputBinding:
      position: 1
      prefix: --othresh
  pthresh:
    type: string?
    inputBinding:
      position: 1
      prefix: --pthresh
  read_length:
    type: string?
    inputBinding:
      position: 1
      prefix: --read_length
  out_dir:
    type: string?
    default: uproc-out
    inputBinding:
      position: 1
      prefix: --out_dir

outputs:
  out_dir:
    type: Directory
    outputBinding:
      glob: $(inputs.out_dir)
