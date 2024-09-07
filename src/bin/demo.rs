use idiomatic_rust::*;

fn main() -> Result<Para, ParaError> {
   "15 $".parse::<Para>()
}
