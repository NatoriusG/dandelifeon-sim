# Dandelifeon Sim

## Description

The dandelifeon flower in Botania generates resources based on a version of Conway's Game of Life. Botania introduces a new piece of data for each cell, age, which determines how valuable the resources provided by the cell are. This program implements Conway's Game of Life with that additional age attribute in an extremely simple terminal format.

## Why?

Given the complexity of Conway's Game, determining the success of a given starting state for the dandelifeon board is a matter of trial and error. However, experimenting with board states requires repeatedly crafting cell blocks to use, since the initial cells are destroyed by the progression of the Game. I created this simulator to allow me to experiment and validate possible initial board states without wasting resources.

## Features

* [x] Correct simulation of Conway's Game of Life with age data
* [x] Rendering of board state for visual inspection
* [x] Loading an initial board state from a file
* [ ] Simulation of dandelifeon collection mechanics