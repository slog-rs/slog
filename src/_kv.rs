/// Owned KV
///
/// Zero, one or more owned key-value pairs.
pub struct OwnedKV(
    #[doc(hidden)]
    /// The exact details of that it are not considered public
    /// and stable API. `slog_o` or `o` macro should be used instead
    /// to create `OwnedKV` instances.
    pub Box<KV + Send + Sync + 'static>,
);

/// BorrowedKV
///
/// Zero, one or more borrowed key-value pairs.
pub struct BorrowedKV<'a> (
    #[doc(hidden)]
    /// The exact details of it function are not considered public
    /// and stable API. `log` and other macros should be used instead
    /// to create `BorrowedKV` instances.
    pub &'a KV,
);

impl<'a> BorrowedKV<'a> {
    /// Iterate over every single `KV` in the the group
    pub fn iter(&self) -> BorrowedKVIterator<'a> {
        BorrowedKVIterator {
            cur: self.0
        }
    }
}

/// Iterato over `BorrowedKV`
pub struct BorrowedKVIterator<'a> {
    cur : &'a KV
}


impl<'a> Iterator for BorrowedKVIterator<'a> {
    type Item = &'a KV;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((head, tail)) = self.cur.split_first() {
            self.cur = tail;
            return Some(head);
        } else {
            None
        }
    }
}

struct OwnedKVListNode {
    next_node: Option<Arc<OwnedKVListNode>>,
    kv: OwnedKV,
}

/// Chain of `SyncMultiSerialize`-s of a `Logger` and its ancestors
#[derive(Clone)]
pub struct OwnedKVList {
    next_list: Option<Arc<OwnedKVList>>,
    node: Arc<OwnedKVListNode>,
}

impl fmt::Debug for OwnedKVList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));
        let mut i = 0;

        {
            let mut as_str_ser = AsFmtSerializer(|key, _val| {
                if i != 0 {
                    try!(write!(f, ", "));
                }

                try!(write!(f, "{}", key));
                i += 1;
                Ok(())
            });
            let record_static = RecordStatic {
                level: Level::Trace,
                file: "",
                line: 0,
                column: 0,
                function: "",
                module: "",
                target: "",
            };
            let record = Record::new(&record_static, format_args!(""), BorrowedKV(&STATIC_TERMINATOR_UNIT));

            for i in self.iter_groups() {
                try!(i.serialize(&record, &mut as_str_ser)
                     .map_err(|_| fmt::Error));
            }
        }

        try!(write!(f, ")"));
        Ok(())
    }
}

impl OwnedKVList {
    /// New `OwnedKVList` node without a parent (root)
    fn root(values: OwnedKV) -> Self {
        OwnedKVList {
            next_list: None,
            node: Arc::new(OwnedKVListNode {
                next_node: None,
                kv: values,
            }),
        }
    }

    /// New `OwnedKVList` node with an existing parent
    fn new(values: OwnedKV, next_node: Arc<OwnedKVListNode>) -> Self {
        OwnedKVList {
            next_list: None,
            node: Arc::new(OwnedKVListNode {
                next_node: Some(next_node),
                kv: values,
            }),
        }
    }

    fn append(&self, other: &OwnedKVList) -> OwnedKVList {
        OwnedKVList {
            next_list: Some(Arc::new(if let Some(ref next) = self.next_list {
                next.append(other)
            } else {
                other.clone()
            })),
            node: self.node.clone(),
        }
    }

    /// Iterate over every single `KV` of `OwnedKVList`
    ///
    /// The order is reverse to how it was built. Eg.
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let drain = slog::Discard;
    ///     let root = slog::Logger::root(drain, o!("k1" => "v1", "k2" => "k2"));
    ///     let _log = root.new(o!("k3" => "v3", "k4" => "v4"));
    /// }
    /// ```
    ///
    /// Will produce `OwnedKVList.iter()` that returns `k4, k3, k2, k1`.
    pub fn iter_single(&self) -> OwnedKVListIterator {
        OwnedKVListIterator::new(self)
    }

    /// Iterate over every `OwnedKV` of `OwnedKVList`
    ///
    /// This is generally faster aproach
    pub fn iter_groups(&self) -> OwnedKVListGroupIterator {
        OwnedKVListGroupIterator::new(self)
    }
}

/// Iterator over `OwnedKVList`-s
///
/// The `&KV` returned corespond to `OwnedKV`s,
/// meaning they can serialize to multiple key-value
/// pairs, and can be iterated further using
/// `KV::split_first`.
pub struct OwnedKVListGroupIterator<'a> {
    next_list: Option<&'a OwnedKVList>,
    next_node: Option<&'a OwnedKVListNode>,
}

impl<'a> OwnedKVListGroupIterator<'a> {
    fn new(list: &'a OwnedKVList) -> Self {
        OwnedKVListGroupIterator {
            next_list: Some(list),
            next_node: None,
        }
    }
}

impl<'a> Iterator for OwnedKVListGroupIterator<'a> {
    type Item = &'a OwnedKV;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(node) = self.next_node.take() {
                self.next_node = node.next_node.as_ref().map(|next| &**next);
                return Some(&node.kv);
            }
            if let Some(list) = self.next_list.take() {
                self.next_node = Some(&*list.node);
                self.next_list = list.next_list.as_ref().map(|next| &**next);
                continue;
            }
            return None;
        }
    }
}

/// Iterator over `OwnedKVList`-s
///
/// The `&KV` returned are guaranteed to produce only single key-value
pub struct OwnedKVListIterator<'a> {
    next_list: Option<&'a OwnedKVList>,
    next_node: Option<&'a OwnedKVListNode>,
    cur: Option<&'a KV>,
}

impl<'a> OwnedKVListIterator<'a> {
    fn new(list: &'a OwnedKVList) -> Self {
        OwnedKVListIterator {
            next_list: Some(list),
            next_node: None,
            cur: None,
        }
    }
}

impl<'a> Iterator for OwnedKVListIterator<'a> {
    type Item = &'a KV;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(x) = self.cur.take() {
                if let Some((head, tail)) = x.split_first() {
                    self.cur = Some(tail);
                    return Some(head);
                }
            }
            if let Some(node) = self.next_node.take() {
                self.cur = Some(&*node.kv.0);
                self.next_node = node.next_node.as_ref().map(|next| &**next);
                continue;
            }
            if let Some(list) = self.next_list.take() {
                self.next_node = Some(&*list.node);
                self.next_list = list.next_list.as_ref().map(|next| &**next);
                continue;
            }
            return None;
        }
    }
}
