MAIN=./main.rs
RID_DERIVES=../rid-derive/src/*.rs

MAIN_EXPANDED=./main-expanded.rs

BINDING_C=./binding.h

PRINT_CODE=pygmentize -O style=monokai -f console256 -g

bindgen: $(BINDING_C)
	@echo
	@echo $(BINDING_C)
	@ $(PRINT_CODE) $(BINDING_C)
	@ $(MAKE) clean

	
$(BINDING_C): $(MAIN_EXPANDED)
	cbindgen -c cbindgen.toml $< -o $(BINDING_C)

$(MAIN_EXPANDED): $(MAIN) $(RID_DERIVES)
	cargo expand > $@

clean:
	@rm -f $(MAIN_EXPANDED) $(BINDING_C) $(BINDING_DART)

.PHONY:
	bindgen
	clean-expand
	clean
