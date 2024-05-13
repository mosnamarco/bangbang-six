# Welcome to the bangbang_six README!
Author: Marco D. Mosna
Purpose: Final Project Simple TCP Connection demonstration

## Required  setup
1. Unix based OS (Preferably Linux)
2. Rust compiler (rustc and/or Cargo)
3. Terminal (for client connection, server can be run just by executing the executable)

## Scope and limitation
1. Game logic is not 100% done
2. TCP connection works
3. Game is threaded, but not fully optimized (might suffer performance issues)
4. Game UI is not 100% done (tightly related to game logic)
5. Resource is not embedded in the binary (needs to be put in a folder, already provided in bb6.zip, here.)

## Guarantees
1. Will handle TCP connection over LAN
2. Will print the string "YOU WIN" and "YOU LOSE" as a substring (this depends on game state)
3. Will not need Rust compiler to run (Built in rust but already has pre-built binary along with it's resource in a folder)
