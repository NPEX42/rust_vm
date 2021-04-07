mod rust_vm;

fn main() {
    println!("Hello, world!");

    let mut vm = rust_vm::RustVM::new();

    vm.clock();
}

#[cfg(test)]
#[test]
fn test_init() {

}



