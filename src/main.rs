struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: i32,
    height: i32,
}

struct Tree {
    root: Option<Box<Node>>
}

impl Tree {

    fn new(data: i32) -> Tree {
        let root = Some(Box::new(Node {
            data: data,
            left: None,
            right: None,
            height: 0
        }));
        Tree {
            root
        }
    }

    // TODO Print In form of piramid
    // fn print_peramid(&self) {
    //     match self.root {
    //         Some(val) => {
                
    //         },
    //         None => {
    //             println!("Empty Treee");
    //         }
    //     }
    // }

    fn rebalance (node : Box<Node> ) -> Box<Node> {
        let left_height = Node::ge_height(&node.left);
        let right_height = Node::ge_height(&node.right);
        if (left_height > right_height) {
            // left rotate
            let left_child = node.left.as_ref();
            let left_left_height= left_child.as_ref().map_or(0, |f| {
                Node::ge_height(&f.left)
            });
            let left_right_height = left_child.as_ref().map_or(0, |f| {
                Node::ge_height(&f.right)
            });
            if (left_left_height > left_right_height) {
                //left left rotate
                return Node::left_rotate(node);
            } else {
                return Node::left_right_rotate(node);
                // left right rotate
            }
        } else {
            // right rotate
            let righ_child = node.right.as_ref();
            let right_left_height= righ_child.as_ref().map_or(0, |f| {
                Node::ge_height(&f.left)
            });
            let right_right_height = righ_child.as_ref().map_or(0, |f| {
                Node::ge_height(&f.right)
            });
            if (right_left_height > right_right_height) {
                //right left rotate
                return Node::right_left_rotate(node);
            } else {
                return Node::right_rotate(node);
                // right right rotate
            }
        }
    }

    fn insert(&mut self, data: i32) {
        let root = self.root.take();
        self.root = Some(Node::insert_node( root, data));
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = Node::to_string(&self.root);
        write!(f, "{}",str)
    }
}

impl Node {
    fn new(data: i32) -> Box<Node> {
        Box::new(Node {
            data: data,
            height: 0,
            left: None,
            right: None,
        })
    }

    fn left_rotate (mut node : Box<Node>) -> Box<Node> {
        // This should never happen
        node.height -= 2;
        let mut new_head: Box<Node> = node.left.take().expect("Node New Head is Empty");
        let value_to_insert_after : Option<Box<Node>> = new_head.right.take();
        new_head.right = Some(node);
        
        if value_to_insert_after.is_some() {
            new_head = Self::insert_node(Some(new_head), value_to_insert_after.unwrap().data);
        }
        new_head
    }
    fn left_right_rotate (mut node : Box<Node>) -> Box<Node> {
        // right rotate on left
        let new_left = Self::right_rotate(node.left.expect("Left is empty in left right rotate"));
        node.left = Some(new_left);
        Self::left_rotate(node)
    }
    fn right_left_rotate (mut node : Box<Node>) -> Box<Node> {
        let new_right = Self::left_rotate(node.right.expect("Right is empty in right left rotate"));
        node.right = Some(new_right);
        Self::right_rotate(node)
    }
    fn right_rotate (mut node : Box<Node>) -> Box<Node> {
        // This should never happen
        node.height -= 2;
        let mut new_head: Box<Node> = node.right.take().expect("Node New Head is Empty");
        let value_to_insert_after : Option<Box<Node>> = new_head.left.take();
        new_head.left = Some(node);
        if value_to_insert_after.is_some() {
            new_head = Self::insert_node(Some(new_head), value_to_insert_after.unwrap().data);
        }
        new_head
    }

    fn ge_height (node:& Option<Box<Node>>) -> i32 {
        node.as_ref().map_or(0, |f| {return f.height + 1 })
    }

    fn to_string(node: & Option<Box<Node>>) -> String {
        match node {
            Some(v) => {
                let mut string = self::Node::to_string(&v.left);
                string.push_str(&" [");
                string.push_str(&v.data.to_string());
                string.push(' ');
                string.push_str(&v.height.to_string());
                string.push_str(&" ]");
                string.push_str(&self::Node::to_string(&v.right));
                return string;
            },
            None => {
                " ".to_string()
            }
        }
    }

    fn insert_node ( mut node :  Option<Box<Node>>, data: i32) -> Box<Node> {
        match node {
            Some(mut value) => {
                if value.data == data {
                    return value;
                }
                if value.data > data  {
                    value.left = Some(Self::insert_node(value.left, data)); 
                } else {
                    value.right = Some(Self::insert_node( value.right.take(), data));
                }
                let left_height = Node::ge_height(&value.left);
                let right_height = Node::ge_height(&value.right);
                value.height = std::cmp::max(left_height, right_height);
                if (left_height - right_height).abs() > 1 {
                    value = Tree::rebalance(value);
                };
                return value;
            }
            None => {
                Node::new(data)
            }
        }
    }
}

fn main() {
    let mut tree = Tree::new(6);
    // tree.insert(7);
    tree.insert(4);
    tree.insert(3);
    tree.insert(2);
    tree.insert(1);
    // tree.insert(0);
    tree.insert(-1);
    tree.insert(-2);
    tree.insert(-3);
    tree.insert(-4);
    tree.insert(-5);
    tree.insert(-6);
    // tree.insert();

    println!("    {}", tree.root.as_ref().unwrap().data);
    // print!("{}  ", tree.root.as_ref().unwrap().left.as_ref().unwrap().data);
    // println!("{}", tree.root.as_ref().unwrap().right.as_ref().unwrap().data);
    // print!("{}  ", tree.root.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().data);
    // print!("{}  ", tree.root.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().data);
    println!("{}", tree);

}
