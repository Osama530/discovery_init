target remote: 3333
set print asm-demangle on
break DefaultHandler
break UserHardFault
break rust_being_unwind

monitor arm semihosting enable

load
break main
continue
layout src
step