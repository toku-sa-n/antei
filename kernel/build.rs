fn main() {
    assemble();
}

fn assemble() {
    cc::Build::new().file("src/asm.s").compile("asm");
}
