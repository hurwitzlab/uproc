SINGULARITY = /usr/local/bin/singularity
IMG = uproc-1.2.0.img
DEF = image.def

run:
	sudo $(SINGULARITY) run $(IMG)

clean:
	rm -f $(IMG)

img: clean
	#sudo $(SINGULARITY) create --size $(SIZE) $(IMG)
	#sudo $(SINGULARITY) bootstrap $(IMG_NAME) $(DEF)
	sudo $(SINGULARITY) build $(IMG) $(DEF)

shell:
	sudo $(SINGULARITY) shell --writable -B $(shell pwd):/tmp $(IMG)

dolphin:
	sudo $(SINGULARITY) exec -B /data:/work -B $(HOME):/home1 $(IMG) run_uproc \
	-q /work/kyclark/dolphin/fasta/ \
	-o /work/kyclark/uproc/dolphin-out \
	--uproc_db_dir /work/kyclark/uproc/db \
	--uproc_model_dir /work/kyclark/uproc/model/ \
	--annotation_dir /work/kyclark/uproc/annotations
