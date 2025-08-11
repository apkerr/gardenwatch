use assert_cmd::Command;

#[test]
fn runs() -> Result<(), Box<dyn std::error::Error>> {
   Command::cargo_bin("gardenwatch")?
    .arg("init");
   

   Ok(())
}