# Unicode Clusters

Unicode Clusters is a library that support variable width unicode characters as single items, allowing for array like indexing etc.


```rust
#[test]
fn example() {
	let input = "AȜनमस्ते";

	let gcs = GraphemeCluster::graphemes(input);
	assert!(gcs.len() == 6, "length");

	assert_eq!(gcs[0].as_string(), "A");

	assert_eq!(gcs[1].as_string(), "Ȝ");
	assert_eq!(gcs[2].as_string(), "न");
	assert_eq!(gcs[3].as_string(), "म");
	assert_eq!(gcs[4].as_string(), "स्");
	assert_eq!(gcs[5].as_string(), "ते");

	assert_eq!(gcs[0].as_bytes()[..], [65]);
	assert_eq!(gcs[1].as_bytes()[..], [200, 156]);
	assert_eq!(gcs[2].as_bytes()[..], [224, 164, 168]);
	assert_eq!(gcs[3].as_bytes()[..], [224, 164, 174]);
	assert_eq!(gcs[4].as_bytes()[..], [224, 164, 184,	224, 165, 141]);
	assert_eq!(gcs[5].as_bytes()[..], [224, 164, 164,	224, 165, 135]);
}
```