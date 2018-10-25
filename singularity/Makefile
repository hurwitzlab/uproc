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
