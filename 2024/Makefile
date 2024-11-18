ARG := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(ARG):;@:)

check:
	@uv run ruff check

# make test 01
test:
	@uv run pytest -q ./src/day$(ARG)/main.py

# make run 01
run:
	@uv run ./src/day$(ARG)/main.py

# make watch 01
watch:
	@uv run pymon -c -p "*.py;*.txt" ./src/day$(ARG)/main.py
