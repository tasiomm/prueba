// Definición de la estructura
struct Persona {
    nombre: String,
    edad: u32,
}

// Implementación de métodos para la estructura
impl Persona {
    // Método constructor
    fn new(nombre: &str, edad: u32) -> Self {
        Persona {
            nombre: nombre.to_string(),
            edad,
        }
    }

    // Método para mostrar información sobre la persona
    fn presentarse(&self) {
        println!("Hola, mi nombre es {} y tengo {} años.", self.nombre, self.edad);
    }
}

fn main() {
    // Crear una instancia de Persona
    let persona1 = Persona::new("Juan", 30);
    let persona2 = Persona::new("Antonio", 40);

    // Llamar al método presentarse
    persona1.presentarse();
    persona2.presentarse();
}
