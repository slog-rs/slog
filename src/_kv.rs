/// Group of key-value pairs
///
/// The exact details of that function are not considered public
/// and stable API. `slog_o` or `o` macro should be used instead
/// to create `OwnedKVGroup` instances.
pub struct OwnedKVGroup(
    #[doc(hidden)]
    pub Box<ser::SyncMultiKV>,
);

struct OwnedKeyValueListNode {
    next_node: Option<Arc<OwnedKeyValueListNode>>,
    values: OwnedKVGroup,
}

/// Chain of `SyncMultiSerialize`-s of a `Logger` and its ancestors
#[derive(Clone)]
pub struct OwnedKeyValueList {
    next_list: Option<Arc<OwnedKeyValueList>>,
    node: Arc<OwnedKeyValueListNode>,
}

impl fmt::Debug for OwnedKeyValueList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));
        let mut i = 0;

        {
            let mut as_str_ser = ser::AsStrSerializer(|key, val| {
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
            let record = Record::new(&record_static, format_args!(""), &[]);

            for i in self.iter() {
                try!(i.serialize(&record, &mut as_str_ser)
                     .map_err(|_| fmt::Error));
            }
        }

        try!(write!(f, ")"));
        Ok(())
    }
}

impl OwnedKeyValueList {
    /// New `OwnedKeyValueList` node with an existing parent
    #[deprecated]
    pub fn new(values: OwnedKVGroup, parent: OwnedKeyValueList) -> Self {
        OwnedKeyValueList {
            next_list: None,
            node: Arc::new(OwnedKeyValueListNode {
                next_node: Some(parent.node),
                values: values,
            }),
        }
    }

    /// New `OwnedKeyValue` node without a parent (root)
    #[deprecated]
    pub fn root(values: OwnedKVGroup) -> Self {
        OwnedKeyValueList {
            next_list: None,
            node: Arc::new(OwnedKeyValueListNode {
                next_node: None,
                values: values,
            }),
        }
    }

    fn append(&self, other: &OwnedKeyValueList) -> OwnedKeyValueList {
        OwnedKeyValueList {
            next_list: Some(Arc::new(if let Some(ref next) = self.next_list {
                next.append(other)
            } else {
                other.clone()
            })),
            node: self.node.clone(),
        }
    }

    /*
    /// Get the parent node element on the chain of values
    ///
    /// Since `OwnedKeyValueList` is just a chain of `SyncMultiSerialize` instances: each
    /// containing one more more `OwnedKeyValue`, it's possible to iterate through the whole list
    /// group-by-group with `parent()` and `values()`.
    pub fn parent(&self) -> Option<&OwnedKeyValueNode> {
        if let Some(next) = self.node.next_node.as_ref() {
            OwnedKeyValueList {
                    next_list: self.next_list.clone(),
                    node: next.clone(),
                }
                .into()
        } else if let Some(next) = self.next_list.as_ref() {
            OwnedKeyValueList {
                    next_list: next.next_list.clone(),
                    node: next.node.clone(),
                }
                .into()
        } else {
            None
        }
    }

    /// Get the head node `SyncMultiSerialize` values
    pub fn values(&self) -> Option<&ser::SyncMultiSerialize> {
        self.node.values.as_ref().map(|b| &**b)
    }
    */

    /// Iterator over all `OwnedKeyValue`-s in every `SyncMultiSerialize` of the list
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
    /// Will produce `OwnedKeyValueList.iter()` that returns `k4, k3, k2, k1`.
    pub fn iter(&self) -> OwnedKeyValueListIterator {
        OwnedKeyValueListIterator::new(self)
    }

    /// Get a unique stable identifier for this node
    ///
    /// This function is buggy and will be removed at some point.
    /// Please see https://github.com/slog-rs/slog/issues/90
    #[deprecated]
    pub fn id(&self) -> usize {
        &*self.node as *const _ as usize
    }
}

/// Iterator over `OwnedKeyValue`-s
pub struct OwnedKeyValueListIterator<'a> {
    next_list: Option<&'a OwnedKeyValueList>,
    next_node: Option<&'a OwnedKeyValueListNode>,
    cur: Option<&'a ser::KV>,
}

impl<'a> OwnedKeyValueListIterator<'a> {
    fn new(list: &'a OwnedKeyValueList) -> Self {
        OwnedKeyValueListIterator {
            next_list: Some(list),
            next_node: None,
            cur: None,
        }
    }
}

impl<'a> Iterator for OwnedKeyValueListIterator<'a> {
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
                self.cur = Some(&*node.values.0);
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

