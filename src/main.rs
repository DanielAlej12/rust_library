mod data_structures;
mod helpers;

use data_structures::{Stack, Queue, Map, BinaryTree};
use helpers::{Logger, LogLevel, FileManager};

fn main() {
    Logger::log(LogLevel::Info, "Iniciando programa de prueba...");

    // Pila
    let mut s = Stack::new();
    s.push(8); s.push(5); s.push(10); s.push(7);
    println!("\nPrueba de Pila: Apilar y Desapilar"); 
    s.show();
    s.pop();
    s.show();

    // Cola
    let mut q = Queue::new();
    q.enqueue("A"); q.enqueue("B"); q.enqueue("C"); q.enqueue("D");
    println!("\nPrueba de Cola: Encolar y Desencolar"); 
    q.show();
    q.dequeue();
    q.show();

    // Mapa
    let mut m = Map::new();
    m.insert("EST_01", "Programacion II");
    m.insert("EST_02", "Bases de Datos");
    m.insert("EST_03", "Sistemas Operativos");
    m.insert("EST_04", "Desarrollo Web");
    println!("\nPrueba de Mapa: Insercion y Busqueda"); 
    println!("Contenido del Mapa:");
    m.show_all();

    let key_buscada = "EST_01";
    match m.get(&key_buscada) {
        Some(valor) => Logger::log(LogLevel::Info, &format!("Consulta exitosa: {} tiene asignado {}", key_buscada, valor)),
        None => Logger::log(LogLevel::Warning, &format!("La llave {} no existe", key_buscada)),
    }

    // Árbol
    let mut t = BinaryTree::new();
    for n in [50, 30, 70, 20, 40] { t.insert(n); }
    println!("\nPrueba de Arboles: Insercion en Impresion en distinto orden");  
    print!("Árbol (In-Order): ");
    t.print_in_order();
    println!();
    print!("Árbol (Pre-Order): ");
    t.print_pre_order();
    println!();
    print!("Árbol (Post-Order): ");
    t.print_post_order();
    println!();
    println!();
 
    // Write
    let nombre_archivo = "info.txt";
    let contenido_archivo = "Guardando correctamente la informacion en este archivo (.txt)"; 

    match FileManager::write_file(nombre_archivo, contenido_archivo) {
        Ok(_) => Logger::log(LogLevel::Info, "Archivo escrito correctamente."),
        Err(e) => Logger::log(LogLevel::Error, &format!("Error al escribir: {:?}", e)),
    }

    // Read
    match FileManager::read_file("info.txt") {
        Ok(contenido) => println!("Contenido leído: {}\n", contenido),
        Err(e) => Logger::log(LogLevel::Warning, &format!("No se pudo leer: {:?}", e)),
    }

    Logger::log(LogLevel::Info, "Prueba finalizada.");
}