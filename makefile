converter:
	trunk build

install:
	test -n ${SITE_TARGET}
	rm -r ${SITE_TARGET}/converter
	cp -r ./converter ${SITE_TARGET}/converter

clean:
	cargo clean
	trunk clean
