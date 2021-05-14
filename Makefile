all:
	cd front && npm run build && cd ..
	-powershell Remove-Item back/www -Recurse
	powershell -Command Copy-Item -Force -Recurse -Verbose front/public -Destination back/www
	cd back && make serve

all-linux:
	cd front && npm run build && cd ..
	-rm -rf back/www
	cp -r front/public back/www
	cd back && make serve