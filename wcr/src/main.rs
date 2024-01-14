
fn main() {
    match wcr::get_args().and_then(wcr::run) {
        Err(err) => {
            eprint!("{}", err);
            std::process::exit(1)
        }
        _ => {
            ()
        }
    }
}
