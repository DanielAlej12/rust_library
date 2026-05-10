#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree::Empty
    }

    pub fn insert(&mut self, new_value: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::Node {
                    value: new_value,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                };
            }
            BinaryTree::Node { value, left, right } => {
                if new_value < *value {
                    left.insert(new_value);
                } else if new_value > *value {
                    right.insert(new_value);
                }
            }
        }
    }

    // Recorrido In-Order: Izquierda -> Raíz -> Derecha
    pub fn print_in_order(&self) where T: std::fmt::Display {
        match self {
            BinaryTree::Empty => (),
            BinaryTree::Node { value, left, right } => {
                left.print_in_order();
                print!("{} ", value);
                right.print_in_order();
            }
        }
    }

    // Recorrido Pre-Order: Raíz -> Izquierda -> Derecha
    pub fn print_pre_order(&self) where T: std::fmt::Display {
        match self {
            BinaryTree::Empty => (),
            BinaryTree::Node { value, left, right } => {
                print!("{} ", value);
                left.print_pre_order();
                right.print_pre_order();
            }
        }
    }

    // Recorrido Post-Order: Izquierda -> Derecha -> Raíz
    pub fn print_post_order(&self) where T: std::fmt::Display {
        match self {
            BinaryTree::Empty => (),
            BinaryTree::Node { value, left, right } => {
                left.print_post_order();
                right.print_post_order();
                print!("{} ", value);
            }
        }
    }
}
