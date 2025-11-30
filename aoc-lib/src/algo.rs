use std::{
	collections::{BinaryHeap, HashMap},
	hash::Hash,
	ops::Add,
};

struct HeapCell<T, W> {
	key: T,
	prio: W,
}

impl<T, W: PartialEq> PartialEq for HeapCell<T, W> {
	fn eq(&self, other: &Self) -> bool {
		self.prio.eq(&other.prio)
	}
}

impl<T, W: Eq> Eq for HeapCell<T, W> {}

impl<T, W: PartialOrd> PartialOrd for HeapCell<T, W> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		other.prio.partial_cmp(&self.prio)
	}
}

impl<T, W: Ord> Ord for HeapCell<T, W> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other.prio.cmp(&self.prio)
	}
}

pub fn dijkstra<T, W, I>(
	start: T,
	get_neighbors: impl Fn(&T) -> I,
) -> (HashMap<T, W>, HashMap<T, T>)
where
	I: Iterator<Item = (T, W)>,
	W: Ord + Default + Clone + Add<Output = W>,
	T: Clone + Eq + Hash,
{
	dijkstra_advance(start, |pos, _| get_neighbors(pos))
}

pub fn dijkstra_advance<T, W, I>(
	start: T,
	mut get_neighbors: impl FnMut(&T, &W) -> I,
) -> (HashMap<T, W>, HashMap<T, T>)
where
	I: Iterator<Item = (T, W)>,
	W: Ord + Default + Clone + Add<Output = W>,
	T: Clone + Eq + Hash,
{
	let mut dist = HashMap::new();
	let mut parent = HashMap::new();
	let mut queue = BinaryHeap::new();

	dist.insert(start.clone(), W::default());
	queue.push(HeapCell {
		key: start,
		prio: W::default(),
	});

	while let Some(HeapCell { key, prio }) = queue.pop() {
		for (neighbor, cost) in get_neighbors(&key, &prio) {
			let d = prio.clone() + cost;
			if dist.get(&neighbor).map(|curr| curr > &d).unwrap_or(true) {
				queue.push(HeapCell {
					key: neighbor.clone(),
					prio: d.clone(),
				});
				dist.insert(neighbor.clone(), d);
				parent.insert(neighbor, key.clone());
			}
		}
	}

	(dist, parent)
}
