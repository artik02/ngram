# List of things left to do (non-exhasutive)

## Fixes

- [X] Fix nonogram editor styles on all platforms
- [ ] Fix nonogram constraints text color on web
- [X] Fix white or bright rgb colors borders to black
- [ ] Fix wonkiness on the editor when updating rows or columns contraints
- [ ] Make nav bar visible on mobile
- [ ] Make drawing multiple blocks work on mobile
- [ ] Make file saving work on web and mobile (file engines!)

## Features

- [X] Add palette editor
    + [X] Add color palette 
        * [X] Add color palette edition
        * [X] Add color brush edition
    + [X] Add nonogram visualization
        * [X] Add nonogram block size option
    + [X] Add nonogram single block edition
    + [X] Add nonogram multiple block edition
    + [X] Add nonogram save button
    + [ ] Add nonogram cleanup button
    + [ ] Add nonogram slide buttons
    + [ ] Add nonogram brush size option
    + [ ] Add nonogram loading (rewrite file handling)
- [X] Add nonogram puzzle solver
    + [X] Develop a usable design
        * [X] Reintegrate color palette
        * [X] Reintegrate nonogram visualization
        * [ ] Show read-only cols and rows on solver
        * [ ] Add an absolut coordinate label
    + [X] Integrate nonogram loading
    + [X] Integrate reactive constraints on solver
    + [ ] Nonogram evaluation function
    + [ ] Add evolutive search to solve nonograms
        * [ ] Develop evolutive algorithms
    + [ ] Apply the ANOVA method
    + [ ] Make the code parallel
    + [ ] Clear off dust of the design
- [ ] Hideable nav bar

## Chores

- [ ] Change nonogram puzzle example
- [X] Rework nonoram editor state with global context
- [ ] Move nonogram components in it's own modules
    + [X] Rework toolbar component
- [x] Move editor to nonogram module
- [ ] Document the code
- [ ] Document in detail
    + [ ] Document the representation of a nonogram
    + [ ] Document the nonogram evalutation function
    + [ ] Document the genetic algorithms
    + [ ] Document the evolutive algorithms
    + [ ] Document the execution and tests
