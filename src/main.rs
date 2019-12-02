use cmd_lib::{info, run_fun, CmdResult};
use std::io::stdin;

fn main() -> CmdResult {
    while true {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        info!("running {}", input);
        let result = run_fun!("{}", input)?;
        info!("{}", result);
    }
    Ok(())
}
