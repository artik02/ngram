# NGRAM

`ngram` is a **cross-platform** application designed to efficiently solve **nonograms** using a **genetic algorithm**. This tool automates the process of solving these logical puzzles, providing quick and accurate solutions. It is ideal for both casual players and developers interested in artificial intelligence applied to puzzles.

Nonograms are graphic logical puzzles where you must fill a grid of cells with different colors based on numeric clues. `ngram` uses a *bio-inspired algorithm* to find optimal solutions to these puzzles efficiently.

> See in another languages:
>
> - [Español de México](/README.es.md)
> - [English of United States](/README.md)

## License

`ngram` is licensed under the [**MIT license**](/LICENSE.md), allowing users to use, modify, and distribute the software freely, provided they adhere to the same terms and conditions outlined in the license.

> Not accepting **contributions** right now.

~~Any **contribution** submitted to the repository must be licensed under the **MIT license**, ensuring that the added code can be used, modified, and distributed by others under the same terms.~~

## Table of Contents

1. [**Installation**](#installation)
2. [**Building**](#building)
3. [**Development**](#development)

## Installation

### Get the package

To install `ngram`, first download the compressed package for your platform from the [**releases**](https://github.com/artik02/ngram/releases) page.

`ngram` is available for the following platforms:
- **Linux**: `ngram-linux-<version>.tar.gz`
- **Windows**: `ngram-windows-<version>.zip`
- **Android**: `ngram-android-<version>.apk`

If you wish to install `ngram` on a different platform, you will need to build the source code.

### Extract the package

Once the compressed package is downloaded, follow the appropriate steps for your platform:

#### Linux

If you downloaded the compressed file for **Linux**, extract it using the following command:

```bash
tar xvf ngram-linux-<version>.tar.gz
```

#### Windows

If you downloaded the compressed file for **Windows**, extract it using the following command in PowerShell:

```powershell
Expand-Archive -Path ngram-windows-<version>.zip -DestinationPath .\
```

### Run the application

Once the package is extracted, run the application for your platform:

#### Linux

Open a terminal and run the following command:

```bash
./ngram
```

#### Windows

In PowerShell, run the following command:

```powershell
.\ngram.exe
```

#### Android

Install the `ngram-android-<version>.apk` app and click the app icon from your device's home screen.

## Building

### Requirements

To build `ngram` from the source code, you will need to have the following components installed:

- Version control system [**Git**](https://git-scm.com/downloads "Git is a distributed version control system used for managing source code and collaboration in software projects.")
- Programming language [**Rust**](https://www.rust-lang.org/learn/get-started "Rust is a programming language focused on safety, performance, and concurrency, ideal for low-level systems and high-performance applications.")
- Framework for creating cross-platform applications [**Dioxus**](https://dioxuslabs.com/learn/0.6/getting_started/ "Dioxus is a framework for building desktop and mobile applications using Rust. It allows creating native interfaces for different platforms.") (requires *Rust*)
- Runtime environment [**Node.js**](https://nodejs.org/en/download/package-manager "Node.js is a runtime for JavaScript on the server side, based on Chrome's V8 engine, ideal for scalable and high-performance applications.")
- Package manager [**npm**](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm "npm is the default package manager for Node.js, used to install, update, and manage dependencies in JavaScript projects.") (requires *Node.js*)

If you wish to build `ngram` for mobile devices (Android or iOS), you should also follow the [**mobile devices guide**](https://dioxuslabs.com/learn/0.6/guides/mobile) provided by Dioxus.

### Clone the repository

To start building, first clone the `ngram` repository using the following command:

```bash
git clone https://github.com/artik02/ngram.git
```

### Execute the Tailwind CSS compiler

Run the **TailwindCSS** compiler in the main repository folder to generate it's build artifacts:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css
```

### Supported platforms

`ngram` is built using the Dioxus framework, so make sure the target platform is compatible. To see the supported platforms, run the following command:

```bash
dx help build
```

Look for the `--platform` option section for information about available platforms.

### Build the application

Once you’ve selected the desired platform, run the following command, replacing `<platform>` with the corresponding platform:

```bash
dx build --release --platform <platform>
```

The first time you build the application, the process may take longer as dependencies need to be downloaded and compiled. However, subsequent builds will be faster due to caching.

## Development

If you want to contribute to the development of `ngram`, make sure you have all the [**requirements**](#requirements) and have [**cloned the repository**](#clone-the-repository).

To start developing and immediately see changes, run the **TailwindCSS** compiler in the main repository folder in the background:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Next, run `Dioxus` in the background with the following command to see the changes reflected:

```bash
dx serve
```

This command will start a development server that lets you view the application while making changes to the code.
