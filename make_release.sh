#!/bin/bash

# MIT LICENSE
#
# Copyright 2024 artik02
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the “Software”), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
# of the Software, and to permit persons to whom the Software is furnished to do
# so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

set -e  # Salir inmediatamente si un comando regresa un valor que no sea cero

# Obtener la versión desde el argumento
version=$1

# Verificar si la versión es proporcionada y si sigue el formato de versionado semántico
if [[ -z "$version" || ! "$version" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
    echo "Error: El formato de versión es inválido. Por favor, proporciona una versión válida en formato semántico, como: v0.0.0 o v0.0.0-alpha"
    exit 1
fi

# Obtener la ruta absoluta del script
directory=$(realpath "$(dirname "$0")")

# Directorios para las versiones release y latest
release_dir="$directory/target/dx/ngram/release"
latest_dir="$directory/target/latest"

# Función para limpiar el directorio 'latest'
clean_latest_dir() {
    echo "Limpiando el directorio '$latest_dir'..."
    echo "Eliminando directorio si existe: rm -r \"$latest_dir\" 2>/dev/null || true"
    rm -r "$latest_dir" 2>/dev/null || true  # Ignorar error si el directorio no existe
    echo "Creando nuevo directorio '$latest_dir'..."
    mkdir -p "$latest_dir"
}

# Función para construir y copiar los archivos de release para cada plataforma
build_and_copy() {
    platform=$1
    opt_release=$2
    opt_latest=$3

    echo "Iniciando compilación para la plataforma '$platform'..."
    echo "Comando de construcción: dx build --release --platform \"$platform\""
    # Ejecutar la construcción y esperar a que termine
    if ! dx build --release --platform "$platform"; then
        echo "Error: La compilación falló para la plataforma '$platform'."
        exit 1
    fi

    echo "Copiando archivos de release para la plataforma '$platform' a '$latest_dir'..."
    echo "Comando: cp -r \"$release_dir/$platform/app$opt_release\" \"$latest_dir/ngram-$platform-$version$opt_latest\""
    # Después de la construcción exitosa, copiar los archivos de release
    cp -r "$release_dir/$platform/app$opt_release" "$latest_dir/ngram-$platform-$version$opt_latest"
}

# Función para comprimir los archivos de release
compress_and_ext() {
    command=$1
    extension=$2

    # Cambiar al directorio 'latest'
    echo "Cambiando al directorio '$latest_dir'..."
    cd "$latest_dir"
    # Comprimir los archivos de release
    echo "Comando de compresión: $command \"ngram-$platform-$version$extension\" \"ngram-$platform-$version\""
    $command "ngram-$platform-$version$extension" "ngram-$platform-$version"
    # Volver al directorio inicial
    echo "Volviendo al directorio anterior..."
    cd -
    # Eliminar los archivos originales
    echo "Eliminando directorio temporal 'ngram-$platform-$version'..."
    rm -r "$latest_dir/ngram-$platform-$version"
}

# Función para generar la release para Linux
make_linux() {
    echo "Generando release para Linux..."
    build_and_copy "linux"
    compress_and_ext "tar cvf" ".tar.gz"
}

# Función para generar la release para Windows
make_windows() {
    echo "Generando release para Windows..."
    build_and_copy "windows"
    compress_and_ext "zip -r" ".zip"
}

# Función para generar la release para Android
make_android() {
    echo "Generando release para Android..."
    build_and_copy "android" "/app/build/outputs/apk/debug/app-debug.apk" ".apk"
}

# Creación principal de la release
echo "Iniciando la creación de la release..."

clean_latest_dir

# Generación de las releases para cada plataforma
make_linux
make_windows
make_android

echo "¡La creación de la release se completó exitosamente!"
