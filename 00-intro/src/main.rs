#[path="basic.rs"] mod basic;
#[path="intermediate.rs"] mod intermediate;
#[path="advanced.rs"] mod advanced;
#[path="scratch.rs"] mod scratch;

fn main() {
    basic::basic_rust();
    intermediate::intermediate_rust();
    advanced::advanced_rust();
    scratch::scratch_sheet();
}