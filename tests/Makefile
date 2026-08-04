.PHONY: all clean regress

all: sigma_test_runner

sigma_test_runner: sigma_test_runner.cpp
	g++ -O2 -I.. -I../include -I../klib/include sigma_test_runner.cpp -o sigma_test_runner

regress: sigma_test_runner
	./sigma_test_runner > regress-kernel.log 2>&1

clean:
	rm -f sigma_test_runner regress-*.log
