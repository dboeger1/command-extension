use print_command::*;
use std::process::Command;


#[test]
fn test_success_print_on_success_false() -> Result<(), Error> {
    let mut john_cena = Command::new("echo");
    john_cena.arg("You can't see me!");
    run_and_print(&mut john_cena, false)
}

#[test]
fn test_success_print_on_success_true() -> Result<(), Error> {
    let mut dutch = Command::new("echo");
    dutch.arg("I'm here! Come on! Do it!");
    run_and_print(&mut dutch, true)
}

#[test]
fn test_failure_print_on_success_false() {
    let mut pinocchio = Command::new("test");
    pinocchio.args([
        "-d",
        "/dev/null",
    ]);
    let result = run_and_print(&mut pinocchio, false);
    assert!(result.is_err());
}

#[test]
fn test_failure_print_on_success_true() {
    let mut pinocchio = Command::new("test");
    pinocchio.args([
        "-d",
        "/dev/null",
    ]);
    let result = run_and_print(&mut pinocchio, true);
    assert!(result.is_err());
}
