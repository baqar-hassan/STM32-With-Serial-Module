# STM32-With-Serial-Module

Controlling STM32 LEDs from serial module using USART protocol.


## USART
The microcontroller has a peripheral called USART, which stands for Universal Synchronous/Asynchronous Receiver/Transmitter. This peripheral can be configured to work with several communication protocols like the serial communication protocol. Make sure this protocol involves two data lines: TX and RX.

We'll be using the pin PA9 as the microcontroller's TX line and PA10 as its RX line. In other words, the pin PA9 outputs data onto its wire whereas the pin PA10 listens for data on its wire.

The serial module also has TX and RX pins. We'll have to cross these pins: that is connect the microcontroller's TX pin to the serial module's RX pin and the micro's RX pin to the serial module's TX pin. The wiring diagram below shows all the necessary connections.


### STM32F3DISCOVERY & Serial Module Connectivity
![STM32F3DISCOVERY & Serial Module](https://docs.rust-embedded.org/discovery/assets/f3-serial.png)


These are the recommended steps to connect the microcontroller and the serial module:
- Close `OpenOCD` and `itmdump`
- Disconnect the `USB cables` from the F3 and the serial module.
- Connect one of F3 GND pins to the GND pin of the serial module using a female to male (F/M) wire. Preferably, a black one.
- Connect the PA9 pin on the back of the F3 to the RXI pin of the serial module using a F/M wire.
- Connect the PA10 pin on the back of the F3 to the TXO pin of the serial module using a F/M wire.
- Now connect the USB cable to the F3.
- Finally connect the USB cable to the `Serial module`.
- Re-launch `OpenOCD` and `itmdump`

Everything's wired up! Let's proceed to send data back and forth.

