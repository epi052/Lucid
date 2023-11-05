# Lucid
An educational Bochs-based snapshot fuzzer project

## Misc
Bochs: https://bochs.sourceforge.io/

Blog: https://h0mbre.github.io/New_Fuzzer_Project/#

Better README coming soon, I apologize!

## Build
`git clone https://github.com/h0mbre/Lucid/`
`cd Lucid`
`gcc test.c -o test -static-pie`
<change `BOCHS_IMAGE` constant in `loader.rs` to `test` path>
`cargo run --release -- --bochs-args -AAAAAAAA`
