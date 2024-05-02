// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2024 Antmicro <www.antmicro.com>

use core::panic::PanicInfo;
use kernel::debug;
use veer_el2::io::Writer;

use crate::CHIP;
use crate::PROCESSES;
use crate::PROCESS_PRINTER;

static mut WRITER: Writer = Writer {};

/// Panic handler.
#[cfg(not(test))]
#[no_mangle]
#[panic_handler]
pub unsafe fn panic_fmt(pi: &PanicInfo) -> ! {
    let writer = &mut WRITER;

    debug::panic_print(
        writer,
        pi,
        &rv32i::support::nop,
        &PROCESSES,
        &CHIP,
        &PROCESS_PRINTER,
    );

    // By writing 0xff to this address we can exit the simulation.
    // So instead of blinking in a loop let's exit the simulation.
    // write_volatile(0xd0580000 as *mut u8, 0xff);

    loop {}

    unreachable!()
}
