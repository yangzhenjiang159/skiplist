//! 一个SkipList实现，它有着比标准链表更快的随机访问。

/// SkipList提供了一种存储元素的方式，并提供了访问、插入和删除节点方法。
/// 与标准链表不同，SkipList可以通过较少的代价找到一个特定的索引。
pub struct SkipList<T> {
    head: Box<SkipNode<T>>,
    len: usize,
    level_generator: GeometricalLevelGenerator, //几何层级生成器
}
/// SkipList的固有方法
impl<T> SkipList<T> {
    /// 创建一个默认层级16的SkipList
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut skiplist: SkipList<i64> = Skip::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        let lg = GeometricalLevelGenerator::new(16, 1.0 / 2.0);
        SkipList {
            head: Box::new(SkipNode::head(lg.total())),
            len: 0,
            level_generator: lg,
        }
    }

    /// 构造一个新的空的 skiplist，其中包含预期容量的最佳级别数。
    /// 具体来说，它使用 “楼层（log2（容量））” 级别数，确保只有几个节点占据最高级别。
    ///
    /// ```
    /// # Examples
    /// use skiplist::SkipList;
    /// let mut skiplist = SkipList::with_capacity(100);
    /// skiplist.extend(0..100);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let levels = cmp::max(1, (capacity as f64).log2().floor() as usize);
        let lg = GeometricalLevelGenerator::new(levels, 1.0 / 2.0);
        SkipList {
            head: Box::new(SkipNode::head(lg.total())),
            len: 0,
            level_generator: lg,
        }
    }

    /// 清空 skiplist, 移除所有值.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut skiplist = SkipList::new();
    /// skiplist.extend(0..10);
    /// skiplist.clear();
    /// assert!(skiplist.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
        *self.head = SkipNode::head(self.level_generator.total());
    }


    /// 获取 skiplist 元素个数
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut skiplist = SkipList::new();
    /// skiplist.extend(0..10);
    /// assert_eq!(skiplist.len(), 10);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// skiplist 是否为空
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut skiplist = SkipList::new();
    /// assert!(skiplist.is_empty());
    ///
    /// skiplist.push_back(1);
    /// assert!(!skiplist.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    ///将元素插入到给定索引处的 SkipList 中，向下移动所有后续节点。
    ///
    /// # Panics
    /// 如果插入索引大于skiplist的长度，则会感到恐慌
    ///
    /// # Examples
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut skiplist = SkipList::new();
    ///
    /// skiplist.insert(0, 0);
    /// skiplist.insert(5, 1);
    /// assert_eq!(skiplist.len(), 2);
    /// assert!(!skiplist.is_empty());
    /// ```
    pub fn insert(&mut self, value: T, index:usize) {
        if index > self.len {
            panic!("Index out of bounds");
        }
        self.len += 1;
        let new_node = Box::new(SkipNode::new(value, self.level_generator.random()));
        self.head
            .insert_at(new_node, index)
            .unwrap_or_else(|_| panic!("No insertion position is found!"));
    }

    /// Provides a reference to the element at the given index, or `None` if the
    /// skiplist is empty or the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut skiplist = SkipList::new();
    /// assert!(skiplist.get(0).is_none());
    /// skiplist.extend(0..10);
    /// assert_eq!(skiplist.get(0), Some(&0));
    /// assert!(skiplist.get(10).is_none());
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.get_index(index).and_then(|node| node.item.as_ref())
    }

    /// Provides a mutable reference to the element at the given index, or
    /// `None` if the skiplist is empty or the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut skiplist = SkipList::new();
    /// assert!(skiplist.get_mut(0).is_none());
    /// skiplist.extend(0..10);
    /// assert_eq!(skiplist.get_mut(0), Some(&mut 0));
    /// assert!(skiplist.get_mut(10).is_none());
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_index_mut(index)
            .and_then(|node| node.item.as_mut())
    }

    /// Removes and returns an element with the given index.
    ///
    /// # Panics
    ///
    /// Panics is the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use skiplist::SkipList;
    ///
    /// let mut skiplist = SkipList::new();
    /// skiplist.extend(0..10);
    /// assert_eq!(skiplist.remove(4), 4);
    /// assert_eq!(skiplist.remove(4), 5);
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len() {
            panic!("Index out of bounds.");
        } else {
            let node = self.head.remove_at(index).unwrap();
            self.len -= 1;
            node.into_inner().unwrap()
        }
    }


    /// Gets a pointer to the node with the given index.
    fn get_index(&self, index: usize) -> Option<&SkipNode<T>> {
        if self.len() <= index {
            None
        } else {
            self.head.advance(index + 1)
        }
    }

    fn get_index_mut(&mut self, index: usize) -> Option<&mut SkipNode<T>> {
        if self.len() <= index {
            None
        } else {
            self.head.advance_mut(index + 1)
        }
    }
}