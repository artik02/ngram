# NGRAM

`ngram` es una aplicación **multiplataforma** diseñada para resolver **nonogramas** de manera eficiente utilizando un **algoritmo genético**. Esta herramienta automatiza el proceso de resolución de estos acertijos lógicos, permitiendo obtener soluciones rápidas y precisas. Ideal tanto para jugadores casuales como para desarrolladores interesados en la inteligencia artificial aplicada a los puzzles.

Los *nonogramas* son acertijos lógicos de tipo gráfico en los que se debe completar una cuadrícula de celdas con distintos colores, siguiendo pistas numéricas. `ngram` utiliza un *algoritmo bioinspirado*, para encontrar las soluciones óptimas a estos puzzles de manera eficiente.

> Lee en otros lenguajes:
>
> - [Español de México](/README.es.md)
> - [English of United States](/README.md)

## Licencia

`ngram` está licenciado bajo la [**licencia MIT**](/LICENSE.es.md), lo que permite a los usuarios utilizar, modificar y distribuir el software de manera libre, siempre que se mantengan los mismos términos y condiciones establecidos en dicha licencia.

Cualquier **contribución** enviada al repositorio deberá estar licenciada bajo la **licencia MIT**, asegurando que el código añadido pueda ser utilizado, modificado y distribuido por otros bajo los mismos términos.

## Tabla de Contenidos

1. [**Instalación**](#instalación)
1. [**Compilación**](#compilación)
1. [**Desarrollo**](#desarrollo)

## Instalación

### Obtener el paquete

Para instalar `ngram`, primero debes descargar el paquete comprimido para tu plataforma desde la página de [**lanzamientos**](https://github.com/artik02/ngram/releases).

`ngram` está disponible para las siguientes plataformas:
- **Linux**: `ngram-linux-<versión>.tar.gz`
- **Windows**: `ngram-windows-<versión>.zip`
- **Android**: `ngram-android-<versión>.apk`

Si deseas instalar `ngram` en una plataforma diferente, necesitarás compilar el código fuente.

### Extraer el paquete

Una vez descargado el paquete comprimido, sigue los pasos correspondientes según tu plataforma:

#### Linux

Si descargaste el archivo comprimido para **Linux**, descomprímelo utilizando el siguiente comando:

```bash
tar xvf ngram-linux-<versión>.tar.gz
```

#### Windows

Si descargaste el archivo comprimido para **Windows**, descomprímelo con el siguiente comando en PowerShell:

```powershell
Expand-Archive -Path ngram-windows-<versión>.zip -DestinationPath .\
```

### Ejecutar la aplicación

Una vez que el paquete esté extraído, ejecuta la aplicación según tu plataforma:

#### Linux

Abre una terminal y ejecuta el siguiente comando:

```bash
./ngram
```

#### Windows

En PowerShell ejecuta el siguiente comando:

```powershell
.\ngram.exe
```

#### Android

Instala la aplicación `ngram-android-<versión>.apk` y haz clic en el ícono de la app desde la pantalla principal de tu dispositivo.

## Compilación

### Requisitos

Para compilar `ngram` desde el código fuente, necesitarás tener instalados los siguientes componentes:

- Sistema de control de versiones [**Git**](https://git-scm.com/downloads "Git es un sistema de control de versiones distribuido, utilizado para el manejo de código fuente y colaboración en proyectos de software.")
- Lenguaje de programación [**Rust**](https://www.rust-lang.org/learn/get-started "Rust es un lenguaje de programación centrado en la seguridad, rendimiento y concurrencia, ideal para sistemas de bajo nivel y aplicaciones de alto rendimiento.")
- Framework para crear aplicaciones multiplataforma [**Dioxus**](https://dioxuslabs.com/learn/0.6/getting_started/ "Dioxus es un framework para desarrollar aplicaciones de escritorio y móviles usando Rust. Permite crear interfaces nativas en diferentes plataformas.") (requiere *Rust*)
- Entorno de ejecución [**Node.js**](https://nodejs.org/en/download/package-manager "Node.js es un entorno de ejecución para JavaScript en el lado del servidor, basado en el motor V8 de Chrome, ideal para aplicaciones escalables y de alto rendimiento.")
- Gestor de paquetes [**npm**](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm "npm es el gestor de paquetes por defecto para Node.js, utilizado para instalar, actualizar y gestionar dependencias de proyectos JavaScript.") (requiere *Node.js*)

Si deseas compilar `ngram` para dispositivos móviles (Android o iOS), también deberás seguir los pasos de la [**guía de dispositivos móviles**](https://dioxuslabs.com/learn/0.6/guides/mobile) proporcionada por Dioxus.

### Clonar el repositorio

Para comenzar con la compilación, primero clona el repositorio de `ngram` con el siguiente comando:

```bash
git clone https://github.com/artik02/ngram.git
```

### Ejecuta el compilador Tailwind CSS

Usa el compilador **TailwindCSS** en el directorio principal del repositorio para generar sus artefactos:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css
```

### Plataformas soportadas

`ngram` se compila utilizando el framework Dioxus, por lo que deberás asegurarte de que la plataforma de destino sea compatible. Para ver las plataformas soportadas, ejecuta el siguiente comando:

```bash
dx help build
```

Busca la sección de la opción `--platform` para obtener información sobre las plataformas disponibles.

### Compilar la aplicación

Una vez que hayas seleccionado la plataforma deseada, ejecuta el siguiente comando, reemplazando `<plataforma>` por la plataforma correspondiente:

```bash
dx build --release --platform <plataforma>
```

La primera vez que compiles la aplicación, el proceso puede tardar más tiempo debido a que se deben descargar y compilar dependencias. Sin embargo, las compilaciones posteriores serán más rápidas gracias a la caché.

## Desarrollo

Si deseas contribuir al desarrollo de `ngram`, asegúrate de tener todos los [**requisitos**](#requisitos) necesarios y de haber [**clonado el repositorio**](#clonar-el-repositorio).

Para comenzar a desarrollar y ver los cambios de inmediato, ejecuta en segundo plano el compilador de **TailwindCSS** en la carpeta principal del repositorio:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

A continuación, ejecuta `Dioxus` en segundo plano con el siguiente comando para ver los cambios reflejados:

```bash
dx serve
```

Este comando iniciará un servidor de desarrollo que permitirá visualizar la aplicación mientras realizas cambios en el código.
