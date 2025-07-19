# noita-black-hole-sim

## introduction

The repository is a `rust` exercise to help me understand the programming language. I choose the subject of simulating behaviors of multiple "black hole" spells in Noita; [Noita](https://store.steampowered.com/app/881100/Noita/) is my favorite rogue-like game.

### "black hole" in Noita

The idea is to make some basic operations on `rust` since I just started to learn the language. To keep things simple I have chosen to implement the black hole spell in Noita. In a nutshell, it works like this:

- it can attract other objects in the same space; the force of attraction is inverse square to the distance between the black hole and the object;
- it is an object itself with a given mass, thus it can be attracted by other black holes in the same space, and sometimes be thrown away due to the [gravity_assist](https://en.wikipedia.org/wiki/Gravity_assist) effect.

## working schedule

Firstly I am going to write a 2D version program containing multiple black holes. It start with a text-rendered interface.

Then it will be expand to 3D. If I still have the motivation I will try to render it in really graphics.
