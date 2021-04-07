use rust_vm::RustVM;

mod rust_vm;

fn main() {
    println!("Hello, world!");

    let mut vm = rust_vm::RustVM::new();

    vm.clock();

    
}

#[cfg(test)]
#[test]
fn test_init() {
    let mut vm = rust_vm::RustVM::new();
    vm.clock();
    assert_eq!(2, vm.get_ip());
    assert_eq!(rust_vm::INSTRUCTION_BYTES, RustVM::get_instruction_length())
}

#[cfg(test)]
#[test]
fn test_ip_change() {
    let mut vm = rust_vm::RustVM::new();
    vm.clock();
    assert_eq!(2, vm.get_ip());
}

#[cfg(test)]
#[test]
fn test_instruction_length() {
    let mut vm = rust_vm::RustVM::new();
    vm.clock();
    assert_eq!(rust_vm::INSTRUCTION_BYTES, RustVM::get_instruction_length())
}



