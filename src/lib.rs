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
		let bytes = GraphemeCluster::as_gcs(input);
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

	fn as_gcs(input: &str) -> Vec<u8> {
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
}

#[test]
fn it_works() {
	assert_eq!(2 + 2, 4);
}

#[test]
fn singlebyte() {
	let bytes = GraphemeCluster::as_gcs("A");

	assert_eq!([
		65
	], bytes[..], "{:?}", bytes);
}


#[test]
fn doublebyte() {
	let bytes2 = GraphemeCluster::as_gcs("Ȝ");

	assert_eq!([
		200, 156
	], bytes2[..], "{:?}", bytes2);
}