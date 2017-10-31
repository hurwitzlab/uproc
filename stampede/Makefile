APP = uproc-1.2.0
EMAIL = kyclark@email.arizona.edu

clean:
	find . \( -name \*.out -o -name \*.param \) -exec rm {} \;

up: 
	copy_from_manifest.py

template:
	jobs-template $(APP) > job.json

job:
	jobs-submit -F job.json

rmbin:
	rm -rf bin

scriptsgz:
	(cd scripts && tar cvf ../scripts.tgz *)

app:
	apps-addupdate -F app.json

test: clean
	sbatch test.sh

imicrobe:
	sbatch -N 4 -n 4 run.sh -i $(WORK)/iplantc.org/data/imicrobe/projects -o $(WORK)/iplantc.org/data/imicrobe/uproc
