use command_extension::*;
use std::process::Command;


#[test]
fn test_success_print_on_success_false() {
    // Should not print.
    let mut john_cena = Command::new("echo");
    john_cena.arg("John Cena: You can't see me!");

    println!();
    let result = john_cena.execute(&ExecuteParameters {
        print_on_success: false,
    });
    assert!(result.is_ok());
    assert!(result.unwrap().output.status.success());
}

#[test]
fn test_success_print_on_success_true() {
    // Should print.
    let mut dutch = Command::new("echo");
    dutch.arg("Dutch: I'm here! Come on! Do it!");

    println!();
    let result = dutch.execute(&ExecuteParameters {
        print_on_success: true,
    });
    assert!(result.is_ok());
    assert!(result.unwrap().output.status.success());
}

#[test]
fn test_failure_print_on_success_false() {
    // Should print.
    let mut pinocchio = Command::new("cat");
    pinocchio.arg("Pinnochio: I'm a real boy!");

    println!();
    let result = pinocchio.execute(&ExecuteParameters {
        print_on_success: false,
    });
    assert!(result.is_ok());
    assert!(!result.unwrap().output.status.success());
}

#[test]
fn test_failure_print_on_success_true() {
    // Should print.
    let mut pinocchio = Command::new("cat");
    pinocchio.arg("Pinnochio: I'm a real boy!");

    println!();
    let result = pinocchio.execute(&ExecuteParameters {
        print_on_success: true,
    });
    assert!(result.is_ok());
    assert!(!result.unwrap().output.status.success());
}
