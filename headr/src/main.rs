fn main() {
    if let Err(e) = headr::parse_args().and_then(headr::run) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
