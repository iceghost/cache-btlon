use std::mem;

pub struct BinarySearchTree<K, V> {
	root: Link<K, V>,
}

impl<K, V> Default for BinarySearchTree<K, V> {
	fn default() -> Self {
		Self {
			root: Default::default(),
		}
	}
}

impl<K, V> BinarySearchTree<K, V>
where
	K: PartialEq + PartialOrd,
{
	pub fn get(&self, key: &K) -> Option<&V> {
		self.root.0.as_deref()?.get(key)
	}

	pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
		self.root.0.as_deref_mut()?.get_mut(key)
	}

	pub fn set(&mut self, key: K, value: V) -> Option<V> {
		self.root.insert(key, value)
	}

	pub fn delete(&mut self, key: &K) -> Option<V> {
		self.root.delete(key)
	}
}

struct Node<K, V> {
	key: K,
	value: V,
	left: Link<K, V>,
	right: Link<K, V>,
}

impl<K, V> Node<K, V>
where
	K: PartialEq + PartialOrd,
{
	fn new(key: K, value: V) -> Self {
		Self {
			key,
			value,
			left: Link::default(),
			right: Link::default(),
		}
	}

	fn get(&self, key: &K) -> Option<&V> {
		if *key == self.key {
			Some(&self.value)
		} else if *key < self.key {
			self.left.0.as_deref()?.get(key)
		} else {
			self.right.0.as_deref()?.get(key)
		}
	}

	fn get_mut(&mut self, key: &K) -> Option<&mut V> {
		if *key == self.key {
			Some(&mut self.value)
		} else if *key < self.key {
			self.left.0.as_deref_mut()?.get_mut(key)
		} else {
			self.right.0.as_deref_mut()?.get_mut(key)
		}
	}
}

struct Link<K, V>(Option<Box<Node<K, V>>>);

impl<K, V> Link<K, V>
where
	K: PartialEq + PartialOrd,
{
	fn insert(&mut self, key: K, value: V) -> Option<V> {
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

	fn delete(&mut self, key: &K) -> Option<V> {
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

	fn extract_min(&mut self) -> Link<K, V> {
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

pub trait IntoInorderIter {
	type InorderIter: Iterator;
	fn inorder_iter(self) -> Self::InorderIter;
}

impl<'a, K, V> IntoInorderIter for &'a BinarySearchTree<K, V> {
	type InorderIter = InorderIter<'a, K, V>;

	fn inorder_iter(self) -> Self::InorderIter {
		InorderIter {
			stack: Vec::default(),
			current: self.root.0.as_deref(),
		}
	}
}

pub struct InorderIter<'a, K, V> {
	stack: Vec<&'a Node<K, V>>,
	current: Option<&'a Node<K, V>>,
}

impl<'a, K, V> Iterator for InorderIter<'a, K, V> {
	type Item = (&'a K, &'a V);

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if let Some(current) = self.current {
				self.stack.push(current);
				self.current = current.left.0.as_deref();
			} else {
				let ancestor = self.stack.pop()?;
				let elem = (&ancestor.key, &ancestor.value);
				self.current = ancestor.right.0.as_deref();
				return Some(elem);
			}
		}
	}
}

pub trait IntoPreorderIter {
	type PreorderIter: Iterator;
	fn preorder_iter(self) -> Self::PreorderIter;
}

impl<'a, K, V> IntoPreorderIter for &'a BinarySearchTree<K, V> {
	type PreorderIter = PreorderIter<'a, K, V>;

	fn preorder_iter(self) -> Self::PreorderIter {
		PreorderIter {
			stack: Vec::from_iter(self.root.0.as_deref()),
			current: self.root.0.as_deref(),
		}
	}
}

pub struct PreorderIter<'a, K, V> {
	stack: Vec<&'a Node<K, V>>,
	current: Option<&'a Node<K, V>>,
}

impl<'a, K, V> Iterator for PreorderIter<'a, K, V> {
	type Item = (&'a K, &'a V);

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if let Some(current) = self.current {
				let elem = (&current.key, &current.value);
				self.current = current.left.0.as_deref();
				self.stack.extend(&self.current);
				return Some(elem);
			} else {
				let ancestor = self.stack.pop()?;
				self.current = ancestor.right.0.as_deref();
				self.stack.extend(&self.current);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn smoke() {
		let mut tree = BinarySearchTree::default();
		tree.set(1, 1);
		tree.set(0, 0);
		tree.set(3, 3);
		assert_eq!(Some(&1), tree.get(&1));
		assert_eq!(Some(1), tree.delete(&1));
	}

	#[test]
	fn iter_smoke() {
		let mut tree = BinarySearchTree::default();
		tree.set(1, 1);
		tree.set(0, 0);
		tree.set(3, 3);
		assert_eq!(
			&Vec::from_iter(tree.preorder_iter().map(|(k, _)| k).copied()),
			&[1, 0, 3]
		);
		assert_eq!(
			&Vec::from_iter(tree.inorder_iter().map(|(k, _)| k).copied()),
			&[0, 1, 3]
		);
	}
}
