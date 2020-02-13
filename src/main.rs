#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart, _, mut itm, _, mut leds) = aux11::init();
    iprintln!(&mut itm.stim[0], "Author: Baqar Hassan <baqar.hassan92@gmail.com>");

    // Send message to serial modules
    for byte in b"Controll LEDs from 1 to 8 numeric keys: ".iter() {
        // wait until it's safe to write to TDR
        while usart.isr.read().txe().bit_is_clear() {}
        usart.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
    }

    iprintln!(&mut itm.stim[0], "Started Receive Bytes Mode");
    // receive bytes from serial module

    // initializing an array of 8 size and assign false value as all LEDs are turned off 
    let mut leds_status: [bool; 8] = [false; 8];
    loop {
        // wait until it's safe to write to TDR
        while usart.isr.read().rxne().bit_is_clear() {}
        
        // Retrieve the data
        let byte = usart.rdr.read().rdr().bits() as u8;

        // checking the conditions
        if byte >= 49 && byte <= 56 {
            // Send a single character
            usart.tdr.write(|w| w.tdr().bits(u16::from(byte)));

            // Getting the LED index number and convert output into usize
            let index = (byte - 49) as usize;

            // IF index led is turned off, Turn on the LED 
            if leds_status[index] == false {
                leds[index].on();
                leds_status[index] = true;
                iprintln!(&mut itm.stim[0], "LED[{:?}] Turn On", index + 1);
            } else {
                leds[index].off();
                leds_status[index] = false;
                iprintln!(&mut itm.stim[0], "LED[{:?}] Turn Off", index + 1);
            }
        }
    }
}
