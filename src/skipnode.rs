use std::cmp::Ordering;
use std::{
    fmt, iter,
    ptr::{self, NonNull},
};

/// 简写
type Link<T> = Option<NonNull<SkipNode<T>>>;

/// SkipNodes组成了SkipList。SkipList拥有第一个头节点（没有值），每个节点通过“next”拥有下一个节点的所有权。
///
/// 节点有一个“级别”，对应于节点达到的“高度”。
/// “级别”n的节点具有（n+1）个链接到下一个节点，这些节点存储在向量中。
///
/// 由级别0链接的节点应被视为该节点拥有。
///
/// 有一个对应的链路长度向量，其中包含当前节点和下一个节点之间的距离。如果没有下一个节点，则距离是当前节点和最后一个可到达节点之间的距离。
///
/// 最后，每个节点都包含一个指向前一个节点的链接，以防需要向后解析列表。
#[derive(Clone, Debug)]
pub struct SkipNode<V> {
    pub item: Option<V>,
    pub level: usize,
    pub prev: Link<V>,
    pub links: Vec<Link<V>>,
    pub links_len: Vec<usize>,
}