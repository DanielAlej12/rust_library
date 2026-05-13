#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T: Ord + std::fmt::Display + Clone> BinaryTree<T> {
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

    pub fn delete(&mut self, valor: T) {
        *self = self.delete_nodo(valor.clone());
        println!("Nodo eliminado: {}", valor);
    }

    fn delete_nodo(&mut self, valor: T) -> BinaryTree<T> {
        // Extraemos el nodo actual para poder manipularlo
        let actual = std::mem::replace(self, BinaryTree::Empty);

        match actual {
            BinaryTree::Empty => BinaryTree::Empty,
            BinaryTree::Node { value, mut left, mut right } => {
                if valor < value {
                    *left = left.delete_nodo(valor);
                    BinaryTree::Node { value, left, right }
                } else if valor > value {
                    *right = right.delete_nodo(valor);
                    BinaryTree::Node { value, left, right }
                } else {
                    // Nodo encontrado (valor == value)
                    
                    // Caso 1: Es una hoja
                    if let (BinaryTree::Empty, BinaryTree::Empty) = (&*left, &*right) {
                        return BinaryTree::Empty;
                    }
                    
                    // Caso 2: Tiene solo un hijo (Izquierdo vacío)
                    if let BinaryTree::Empty = *left {
                        return *right;
                    }
                    
                    // Caso 3: Tiene solo un hijo (Derecho vacío)
                    if let BinaryTree::Empty = *right {
                        return *left;
                    }

                    // Caso 4: Tiene dos hijos
                    // Buscamos el sucesor (el más a la izquierda del subárbol derecho)
                    let sucesor_valor = right.find_min().unwrap();
                    let nuevo_derecho = right.delete_nodo(sucesor_valor.clone());
                    
                    BinaryTree::Node {
                        value: sucesor_valor,
                        left,
                        right: Box::new(nuevo_derecho),
                    }
                }
            }
        }
    }

    fn find_min(&self) -> Option<T> {
        match self {
            BinaryTree::Empty => None,
            BinaryTree::Node { value, left, .. } => {
                if let BinaryTree::Empty = **left {
                    Some(value.clone())
                } else {
                    left.find_min()
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
