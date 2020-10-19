.PHONY: webserver


stress:
	cd ../cpustress_v1/ && make && cp stress_cpu_x64 ../custom_app/ && cd ../custom_app/

webserver:
	@echo run make stress to build stress_cpu_x64.	
	cd webserver && rustup override set nightly  && cargo build && cd ..

app: 
	cargo build --release

install:
	cp stress_cpu_x64 $(DESTDIR)/usr/bin
	cp target/release/custom_app $(DESTDIR)/usr/bin
	cp webserver/target/debug/webserver $(DESTDIR)/usr/bin