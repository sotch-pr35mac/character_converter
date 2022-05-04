use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use bincode::deserialize_from;
use fst::raw::Fst;

fn build_fst(input: &Path, output: &Path) {
	let rdr = File::open(input).map(BufReader::new).unwrap();
	let dictionary: HashMap<String, String> = deserialize_from(rdr).unwrap();

	let mut keys: Vec<_> = dictionary.keys().collect();
	keys.sort_unstable();
	let fst = Fst::from_iter_set(keys).unwrap();

	let mut wtr = BufWriter::new(File::create(output).unwrap());
	wtr.write_all(fst.as_bytes()).unwrap();
	wtr.flush().unwrap();
}

fn main() {
	// Directory path for build package
	let build_dir = env::var_os("OUT_DIR").unwrap(); // ex) target/debug/build/<pkg>/out
	create_dir_all(&build_dir).unwrap();

	// simplified to traditionnal dictionary path
	let input = Path::new("data/s2t.profile");

	// simplified to traditionnal fst path
	let output = Path::new(&build_dir).join("s2t.fst");

	build_fst(&input, &output);

	// traditionnal to simplified dictionary path
	let input = Path::new("data/t2s.profile");

	// traditionnal to simplified fst path
	let output = Path::new(&build_dir).join("t2s.fst");

	build_fst(&input, &output);
}
