.PHONY: all clean dasm isp isp-fuses isp-flash dw dw-flash

all:
	$(MAKE) -C avr-q test
	$(MAKE) -C avr-q-test all

clean:
	$(MAKE) -C avr-q clean
	$(MAKE) -C avr-q-test clean

dasm:
	$(MAKE) -C avr-q-test dasm

isp:
	$(MAKE) -C avr-q-test isp

isp-fuses:
	$(MAKE) -C avr-q-test isp-fuses

isp-flash:
	$(MAKE) -C avr-q test
	$(MAKE) -C avr-q-test isp-flash

dw:
	$(MAKE) -C avr-q-test dw

dw-flash:
	$(MAKE) -C avr-q test
	$(MAKE) -C avr-q-test dw-flash
