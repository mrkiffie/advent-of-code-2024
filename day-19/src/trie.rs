#[derive(Debug, Default)]
struct Node {
    children: [usize; 26],
    terminal: bool,
}

#[derive(Debug, Default)]
pub struct Trie {
    nodes: Vec<Node>,
}

pub struct PrefixLengthIterator<'s, 't> {
    i: usize,
    current_node: usize,
    trie: &'t Trie,
    s: &'s [u8],
}

impl Iterator for PrefixLengthIterator<'_, '_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.s.len() {
            let c = self.s[self.i];
            debug_assert!(c.is_ascii_lowercase(), "expected lowercase character");

            self.i += 1;

            // normalize character to alphabetic index
            let i = c as usize - 96;
            self.current_node = self.trie.nodes[self.current_node].children[i];
            if self.current_node == 0 {
                return None;
            } else if self.trie.nodes[self.current_node].terminal {
                return Some(self.i);
            }
        }
        None
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            nodes: vec![Node::default()],
        }
    }

    pub fn insert(&mut self, s: &str) {
        // initialize currnet node to the root node
        let mut current_node = 0;
        for c in s.chars() {
            debug_assert!(c.is_ascii_lowercase(), "expected lowercase character");

            // normalize character to alphabetic index
            let i = c as usize - 96;
            // create new node if it doesn't exist
            if self.nodes[current_node].children[i] == 0 {
                self.nodes[current_node].children[i] = self.nodes.len();
                self.nodes.push(Node::default());
            }

            current_node = self.nodes[current_node].children[i];
        }

        self.nodes[current_node].terminal = true;
    }

    pub fn common_prefix_lengths<'s, 't>(&'t self, s: &'s [u8]) -> PrefixLengthIterator<'s, 't> {
        PrefixLengthIterator {
            i: 0,
            current_node: 0,
            trie: self,
            s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_common_prefix_lengths() {
        let mut t = Trie::new();
        dbg!("insert");
        t.insert("trie");
        t.insert("cat");
        t.insert("kit");
        t.insert("cattle");
        t.insert("kitten");
        t.insert("trippy");
        t.insert("category");
        t.insert("kite");
        t.insert("trip");
        t.insert("kin");
        t.insert("kittens");

        dbg!("get");

        assert_eq!(t.common_prefix_lengths(b"ca").collect::<Vec<_>>(), vec![]);
        assert_eq!(t.common_prefix_lengths(b"cat").collect::<Vec<_>>(), vec![3]);
        assert_eq!(
            t.common_prefix_lengths(b"kite").collect::<Vec<_>>(),
            vec![3, 4]
        );
        assert_eq!(
            t.common_prefix_lengths(b"kitten").collect::<Vec<_>>(),
            vec![3, 6]
        );
        assert_eq!(
            t.common_prefix_lengths(b"cattle").collect::<Vec<_>>(),
            vec![3, 6]
        );
        assert_eq!(
            t.common_prefix_lengths(b"kittens").collect::<Vec<_>>(),
            vec![3, 6, 7]
        );
    }
}
