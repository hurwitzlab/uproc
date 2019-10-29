#!/usr/bin/env cwl-runner

doc: "CWL to run UProc batch tool"
id: "RunUProc"
label: "UProc DNA batch tool"

dct:creator:
  "@id": "https://orcid.org/0000-0001-9961-144X"
  foaf:name: Ken Youens-Clark
  foaf:mbox: "mailto:kyclark@email.arizona.edu"

cwlVersion: v1.1
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

$namespaces:
    dct: http://purl.org/dc/terms/
    foaf: http://xmlns.com/foaf/0.1/
