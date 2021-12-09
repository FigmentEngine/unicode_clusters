extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
use std::hash;

#[derive(Debug)]
#[derive(Copy, Clone, Eq, Hash)]
pub enum GraphemeCluster {
	B1(GCBytes<1>),
	B2(GCBytes<2>),
	B3(GCBytes<3>),
	B4(GCBytes<4>),
	B5(GCBytes<5>),
	B6(GCBytes<6>),
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct GCBytes<const N: usize> {
	bytes: [u8; N]
}

impl<const N: usize> GCBytes<N> {
	pub fn new(bytes: Vec<u8>) -> GCBytes<N> {
		let mut buffer = [0u8; N];
		buffer.copy_from_slice(&bytes);

		Self {
			bytes: buffer,
		}
	}
}

impl<const N: usize> PartialEq for GCBytes<N> {
	fn eq(&self, other: &GCBytes<N>) -> bool {
		 self.bytes == other.bytes
	}
}

impl<const N: usize> Eq for GCBytes<N> {}

impl PartialEq<GraphemeCluster> for GraphemeCluster {
	fn eq(&self, other: &GraphemeCluster) -> bool {
		self.as_bytes() == other.as_bytes()
	}
}

impl<const N: usize> hash::Hash for GCBytes<N> {
	fn hash<H: hash::Hasher>(&self, s: &mut H) {
		 self.bytes.hash(s)
	}
}

impl GraphemeCluster {
	pub fn new(input: &str) -> Self {
		let bytes = GraphemeCluster::to_vec(input);
		let len = bytes.len();

		let gc: GraphemeCluster = match len {
			1 => GraphemeCluster::B1(GCBytes::<1>::new(bytes)),
			2 => GraphemeCluster::B2(GCBytes::<2>::new(bytes)),
			3 => GraphemeCluster::B3(GCBytes::<3>::new(bytes)),
			4 => GraphemeCluster::B4(GCBytes::<4>::new(bytes)),
			5 => GraphemeCluster::B5(GCBytes::<5>::new(bytes)),
			6 => GraphemeCluster::B6(GCBytes::<6>::new(bytes)),
			_ => panic!("length is too long for grapheme {}", len)
		};

		gc
	}

	pub fn as_bytes(&self) -> &[u8] {
		match self {
			GraphemeCluster::B1(gc_bytes) => &gc_bytes.bytes,
			GraphemeCluster::B2(gc_bytes) => &gc_bytes.bytes,
			GraphemeCluster::B3(gc_bytes) => &gc_bytes.bytes,
			GraphemeCluster::B4(gc_bytes) => &gc_bytes.bytes,
			GraphemeCluster::B5(gc_bytes) => &gc_bytes.bytes,
			GraphemeCluster::B6(gc_bytes) => &gc_bytes.bytes,
		}
	}

	fn to_vec(input: &str) -> Vec<u8> {
		let gcs = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
		let first_gc = gcs[0];
		let bytes: Vec<u8> = first_gc.bytes().collect();

		bytes
	}

	pub fn graphemes(input: &str) -> Vec<GraphemeCluster> {
		UnicodeSegmentation::graphemes(input, true)
			.map(|c| GraphemeCluster::new(c))
			.collect::<Vec<GraphemeCluster>>()
	}

	pub fn to_string_lossy(self) -> String {
		String::from_utf8_lossy(self.as_bytes()).to_string()
	}
}

use std::fmt;
impl fmt::Display for GraphemeCluster {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_string_lossy())
	}
}

#[test]
fn example() {
	let input = "AȜनमस्ते";

	let gcs = GraphemeCluster::graphemes(input);
	assert!(gcs.len() == 6, "length");

	assert_eq!(gcs[0].to_string_lossy(), "A");

	assert_eq!(gcs[1].to_string_lossy(), "Ȝ");
	assert_eq!(gcs[2].to_string_lossy(), "न");
	assert_eq!(gcs[3].to_string_lossy(), "म");
	assert_eq!(gcs[4].to_string_lossy(), "स्");
	assert_eq!(gcs[5].to_string_lossy(), "ते");

	assert_eq!(gcs[0].as_bytes()[..], [65]);
	assert_eq!(gcs[1].as_bytes()[..], [200, 156]);
	assert_eq!(gcs[2].as_bytes()[..], [224, 164, 168]);
	assert_eq!(gcs[3].as_bytes()[..], [224, 164, 174]);
	assert_eq!(gcs[4].as_bytes()[..], [224, 164, 184,	224, 165, 141]);
	assert_eq!(gcs[5].as_bytes()[..], [224, 164, 164,	224, 165, 135]);
}

#[test]
fn it_works() {
	let bytes = GraphemeCluster::to_vec("A");

	assert_eq!([
		65
	], bytes[..], "{:?}", bytes);

	let bytes2 = GraphemeCluster::to_vec("Ȝ");

	assert_eq!([
		200, 156
	], bytes2[..], "{:?}", bytes2);

	let mut b1 = [0u8; 1];
	b1.copy_from_slice(&bytes);

	let mut b2 = [0u8; 2];
	b2.copy_from_slice(&bytes2);

	println!("{:?}", GraphemeCluster::new("A"));
	println!("{:?}", GraphemeCluster::new("Ȝ"));
	println!("{:?}", GraphemeCluster::new("ते"));
	let key = GraphemeCluster::new("ते");
	println!("{:?}", key.as_bytes());

	use std::collections::HashMap;
	let mut nodes: HashMap<GraphemeCluster, char> = HashMap::new();
	nodes.entry(key).or_insert('a');
	nodes.entry(key).or_insert('b');

	assert_eq!(nodes.get(&key), Some(&'a'), "find inserted item first value");

	let key2 = GraphemeCluster::new("ते");
	assert_eq!(nodes.get(&key2), Some(&'a'), "find existing item with duplicate key");

	let key3 = GraphemeCluster::new("Ȝ");
	println!("{:?}", key3.as_bytes());
	assert_eq!(nodes.get(&key3), None, "don't find non-existing item");
}

#[test]
fn bytes_1() {
	let bytes = GraphemeCluster::to_vec("A");

	assert_eq!([
		65
	], bytes[..], "{:?}", bytes);
}

#[test]
fn bytes_2() {
	let bytes = GraphemeCluster::to_vec("Ȝ");

	assert_eq!([
		200, 156
	], bytes[..], "{:?}", bytes);
}

#[test]
fn bytes_3() {
	let bytes = GraphemeCluster::to_vec("न");

	assert_eq!([
		224, 164, 168
	], bytes[..], "{:?}", bytes);
}

#[test]
fn bytes_4() {
	let bytes = GraphemeCluster::to_vec("𐌰");

	assert_eq!([
		240, 144, 140, 176
	], bytes[..], "{:?}", bytes);
}

#[test]
fn bytes_6() {
	let bytes = GraphemeCluster::to_vec("स्");

	assert_eq!([
		224, 164, 184,
		224, 165, 141
	], bytes[..], "{:?}", bytes);
}

#[test]
fn unicode_compare() {
	let char1 = GraphemeCluster::new("\"");
	let char2 = GraphemeCluster::new("\"");
	assert_eq!(char1, char2, "compare unicode chars");

	let char1 = GraphemeCluster::new("Ȝ");
	let char2 = GraphemeCluster::new("Ȝ");
	assert_eq!(char1, char2, "compare unicode chars wide");
}