.PHONY: all clean regress

all: sigma_test_runner

sigma_test_runner: sigma_test_runner.cpp
	g++ -O2 sigma_test_runner.cpp -o sigma_test_runner

regress: sigma_test_runner
	@echo "[CI] Executing real regression tests via sigma_test_runner..."
	./sigma_test_runner > regress-kernel.log 2>&1 || (cat regress-kernel.log && exit 1)
	@echo "[CI] Regression test execution completed successfully."

clean:
	rm -f sigma_test_runner regress-*.log
