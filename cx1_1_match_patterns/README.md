# Match Patterns

Los Match Patterns son una forma de comparar valores como si fuera un `switch` en otros lenguajes.
Nos permite hacer validaciones

```rs
    let numero = 2;
    match numero {
        0 => println!("Cero"),
        1 => println!("Uno"),
        2 => println!("Dos"),
        _ => println!("_default_"),
    }
```

Aqui "comparamos" si el numero es igual a 0, 1, 2 o cualquier otro condicion utilizando el Wildcard `_`. Esta seria la forma mas "general" de mostrar el patron `match` como un switch.

## Variables

Podemos obtener la variable al dato que estamos matcheando.

```rs
    let numero = 2;
    match numero {
        0 => println!("0"),
        num => println!("{num}"),
    }
```

Y la variable `num` contendra el valor de `2`, y el patron Match ya no nos pediria llenar los casos posibles porque `num` ya es el caso "por defecto" sino se cumplen las otras condiciones.
