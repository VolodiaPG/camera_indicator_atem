all:
	cd front && npm run build && cd ..
	-powershell Remove-Item back/www -Recurse
	powershell -Command Copy-Item -Force -Recurse -Verbose front/public -Destination back/www
	cd back && make serve

prod-linux:
	cd front && npm run build && cd ..
	-rm -rf back/www
	cp -r front/public back/www

all-linux: prod-linux
	cd back && make serve

prod: prod-linux
	cd back && make prod