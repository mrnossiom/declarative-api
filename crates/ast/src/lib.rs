#![allow(unused)]

pub mod types;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1() {
		let v1 = types::Version {
			major: 1,
			minor: 2,
			patch: 3,
		};

		let v2 = types::Version {
			major: 1,
			minor: 2,
			patch: 3,
		};
		assert_eq!(v1, v2);
	}

	#[test]
	fn test_2() {
		let v1 = types::Version {
			major: 1,
			minor: 2,
			patch: 3,
		};
		let v2 = types::Version {
			major: 0,
			minor: 3,
			patch: 4,
		};
		assert!(v1 > v2);
	}
}
