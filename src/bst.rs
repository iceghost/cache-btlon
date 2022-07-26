use std::mem;

pub trait BinarySearchTree {
	type Key;
	type Value;

	fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
	fn set(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
	fn delete(&mut self, key: &Self::Key) -> Option<Self::Value>;
}

#[derive(Default)]
pub struct NaiveBinarySearchTree<K, V> {
	pub root: Link<K, V>,
}

pub struct Node<K, V> {
	pub key: K,
	pub value: V,
	pub left: Link<K, V>,
	pub right: Link<K, V>,
}

impl<K, V> Node<K, V>
where
	K: PartialEq + PartialOrd,
{
	pub fn new(key: K, value: V) -> Self {
		Self {
			key,
			value,
			left: Link::default(),
			right: Link::default(),
		}
	}

	pub fn get(&self, key: &K) -> Option<&V> {
		if *key == self.key {
			Some(&self.value)
		} else if *key < self.key {
			self.left.0.as_deref()?.get(key)
		} else {
			self.right.0.as_deref()?.get(key)
		}
	}
}

pub struct Link<K, V>(Option<Box<Node<K, V>>>);

impl<K, V> Link<K, V>
where
	K: PartialEq + PartialOrd,
{
	pub fn insert(&mut self, key: K, value: V) -> Option<V> {
		let node = if let Some(node) = &mut self.0 {
			node
		} else {
			*self = Link(Some(Box::new(Node::new(key, value))));
			return None;
		};

		if key == node.key {
			let old = mem::replace(&mut node.value, value);
			return Some(old);
		}

		let link = if key < node.key {
			&mut node.left
		} else {
			&mut node.right
		};
		link.insert(key, value)
	}

	pub fn delete(&mut self, key: &K) -> Option<V> {
		let node = self.0.as_mut()?;

		if *key < node.key {
			return node.left.delete(key);
		}
		if *key > node.key {
			return node.right.delete(key);
		}
		// else this is the correct link to delete

		let replacement = node.right.extract_min();
		let old = match replacement.0 {
			// right subtree is empty, replace it with left subtree or None
			None => {
				if let Some(right) = node.left.0.take() {
					self.0.replace(right)
				} else {
					self.0.take()
				}
			}
			Some(mut replacement) => {
				replacement.left = Link(node.left.0.take());
				replacement.right = Link(node.right.0.take());
				self.0.replace(replacement)
			}
		};
		old.map(|node| node.value)
	}

	pub fn extract_min(&mut self) -> Link<K, V> {
		let node = if let Some(link) = &mut self.0 {
			link
		} else {
			return Link(None);
		};

		let mut min = node.left.extract_min();
		if let Link(None) = min {
			let right = node.right.0.take();
			let old_node = if let Some(right) = right {
				self.0.replace(right)
			} else {
				self.0.take()
			};
			min = Link(old_node);
		}
		min
	}
}

impl<K, V> Default for Link<K, V> {
	fn default() -> Self {
		Self(None)
	}
}

impl<K, V> BinarySearchTree for NaiveBinarySearchTree<K, V>
where
	K: PartialEq + PartialOrd,
{
	type Key = K;
	type Value = V;

	fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
		self.root.0.as_ref()?.get(key)
	}

	fn set(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
		self.root.insert(key, value)
	}

	fn delete(&mut self, key: &Self::Key) -> Option<Self::Value> {
		self.root.delete(key)
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn smoke() {
		let mut tree = NaiveBinarySearchTree::default();
		tree.set(1, 1);
		tree.set(0, 0);
		tree.set(3, 3);
		assert_eq!(Some(&1), tree.get(&1));
		assert_eq!(Some(1), tree.delete(&1));
	}
}
