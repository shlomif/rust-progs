
EXES := hello multi_deal_freecell mut-vars deal_freecell string_concat

all: hello multi_deal_freecell mut-vars deal_freecell string_concat

hello: hello.rs
	rustc $<

multi_deal_freecell: multi_deal_freecell.rs
	rustc -L ./src/board_gen_lookup1.rs -O $<

mut-vars: mut-vars.rs
	rustc $<

deal_freecell: deal_freecell.rs
	rustc $<

string_concat: string_concat.rs
	rustc $<

test: all
	runprove t/*.t

clean:
	rm -f $(EXES)
