# Shadowing

El Shadowing es una practica en codigo que nos permite "sombrear"/"ocultar" el valor de una variable creada, permitiendo reutilizar el nombre de la variable pero con un nuevo valor.
⚠ No es lo mismo que mutar una variable.

En Rust, esto se hace usando la palabra clave `let`

Ejemplo:

```rs
let x = 5;
let x = x + 1; // Ahora x es 6
```

Una ventaja del shadowing es que podemos cambiar el tipo de una variable, a diferencia de `mut` no permite cambiar el tipo.

Ejemplo:
Tenemos que los espacios de un string son de tipo `&str` inicialmente, pero luego queremos tener la longitud de espacios que tiene el string que seria de un tipo `usize` (int +), entonces podemos hacer lo siguiente:

```rs
let spaces = "   ";
let spaces = spaces.len(); // Ahora spaces es un número (usize)
```

El shadowing respeta los ámbitos. Si se realiza un shadowing en un ámbito interno, fuera de ese ámbito la variable vuelve a tener el valor (y tipo) que tenía previamente.
Ejemplo:

```rs
fn main() {
    let x = 5;
    let x = x + 1; // x ahora es 6

    {
        let x = x * 2; // x es 12 en este ámbito interno
        println!("El valor de x en el ámbito interno es: {x}");
    }

    println!("El valor de x es: {x}"); // x vuelve a ser 6
}
```
