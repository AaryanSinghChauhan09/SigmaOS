.PHONY: all clean regress

all: sigma_test_runner

<<<<<<< HEAD
sigma_test_runner: sigma_test_runner.cpp
	g++ -O2 -I../include -I../klib/include sigma_test_runner.cpp -o sigma_test_runner
||||||| 65885484f
sigma_test_runner: sigma_test_runner.cpp
<<<<<<< HEAD
	g++ -O2 sigma_test_runner.cpp -o sigma_test_runner
=======
sigma_test_runner: sigma_test_runner.cpp ../kernel/drivers/sigma_driver_manager.cpp ../kernel/drivers/sigma_driver_registry.cpp
	g++ -O2 -I.. -I../include sigma_test_runner.cpp ../kernel/drivers/sigma_driver_manager.cpp ../kernel/drivers/sigma_driver_registry.cpp -o sigma_test_runner
>>>>>>> origin/jules-13571719274074749109-6af93541
||||||| 65885484f
	g++ -O2 sigma_test_runner.cpp -o sigma_test_runner
=======
	g++ -O2 -I../include -I../klib/include sigma_test_runner.cpp -o sigma_test_runner
>>>>>>> origin/jules-4213023701309535613-b11406ba

regress: sigma_test_runner
	./sigma_test_runner > regress-kernel.log 2>&1

clean:
	rm -f sigma_test_runner regress-*.log
