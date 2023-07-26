use std::path::Path;

use super::*;
use hir_lowering::utils;

#[test]
fn test_1() {
	let test_ir = utils::gen_test_ir();
	//dbg!(&test_ir);
	let mut generated_file_list: HashMap<Box<Path>, String> = HashMap::new();
	test_ir.markdown(&mut generated_file_list);
	//dbg!(&generated_file_list);
	generated_file_list.write(Path::new("demo"));
}
