cc = /usr/bin/ld
as = /usr/bin/as
sdk_ver = 14.0
syslibroot = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX$(sdk_ver).sdk"
arch = -arch arm64

hallo: hallo.o
	$(cc) -o hallo hallo.o \
		-lSystem \
		-syslibroot $(syslibroot) \
		-e _start \
		$(arch)

hallo.o: hallo.asm
	$(as) $(arch) -o hallo.o hallo.asm

clean:
	rm -f hallo hallo.o
