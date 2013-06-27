
all: hello mut-vars deal_freecell string_concat

hello: hello.rs
	rustc $<

mut-vars: mut-vars.rs
	rustc $<

deal_freecell: deal_freecell.rs
	rustc $<

string_concat: string_concat.rs
	rustc $<

test: all
	runprove t/*.t
