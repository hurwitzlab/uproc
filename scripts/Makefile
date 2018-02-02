KEGGHITS = AI15merged.fa.uproc.kegg
PFAMHITS = AI15merged.fa.uproc.pfam28

both: clean
	./annotate_uproc_hits.py -p $(PFAMHITS) -k $(KEGGHITS)

kegg: clean
	./annotate_uproc_hits.py -k $(KEGGHITS)

pfam: clean
	./annotate_uproc_hits.py -p $(PFAMHITS)

clean:
	rm -f *.annotated.tab
