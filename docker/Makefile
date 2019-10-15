TAG = hurwitzlab/uproc:1.2.0

build: run_uproc
	cp ../run_uproc/target/release/run_uproc . && docker build --tag=$(TAG) .

run_uproc:
	cd ../run_uproc && cargo build --release

push: 
	docker push $(TAG)
