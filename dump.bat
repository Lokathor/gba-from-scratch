cargo build --examples

arm-none-eabi-objdump target/thumbv4t-none-eabi/debug/examples/ex1 --section-headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std >target/ex1.txt

arm-none-eabi-objdump target/thumbv4t-none-eabi/debug/examples/ex2 --section-headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std >target/ex2.txt
