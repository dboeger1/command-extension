use prun::*;
use std::process::Command;


#[test]
fn test_success_print_on_success_false() {
    // Should not print.
    let mut john_cena = Command::new("echo");
    john_cena.arg("You can't see me!");
    let result = prun(&mut john_cena, false);
    assert!(result.is_ok());
    assert!(result.unwrap().status.success());
}

#[test]
fn test_success_print_on_success_true() {
    // Should print.
    let mut dutch = Command::new("echo");
    dutch.arg("I'm here! Come on! Do it!");
    let result = prun(&mut dutch, true);
    assert!(result.is_ok());
    assert!(result.unwrap().status.success());
}

#[test]
fn test_failure_print_on_success_false() {
    // Should print.
    let mut pinocchio = Command::new("cat");
    pinocchio.arg("I'm a real boy!");
    let result = prun(&mut pinocchio, false);
    assert!(result.is_ok());
    assert!(!result.unwrap().status.success());
}

#[test]
fn test_failure_print_on_success_true() {
    // Should print.
    let mut pinocchio = Command::new("cat");
    pinocchio.arg("I'm a real boy!");
    let result = prun(&mut pinocchio, true);
    assert!(result.is_ok());
    assert!(!result.unwrap().status.success());
}
