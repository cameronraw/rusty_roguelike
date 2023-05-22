# Rusty Roguelike - A Dungeon Crawler game built in Rust
## A study exercise from the book "Hands-on Rust" by Herbert Wolverson.

I'm keeping track of progress through the book in this repo, hoping to finish with a fully functional game built in Rust.

Just about to start Chapter 6

Below is the Short Game Design Document recommended by the book:

## Short Game Design Document for Rusty Roguelike

### Short Description
A dungeon crawler with procedurally generated levels, monsters of increasing difficulty, and turn-based movement.

### Story
The hero's hometown is suffering from a plague of monsters. Welling up from the deep, they seem unstoppable. Legend tells of the Amulet of Yala - Yet Another Lost Amulet - that can be used to stem the tide. After a long night at the tavern, the hero promises to save the day - and sets forth into the dungeon.

### Basic Game Loops
1. Enter the dungeon level.
2. Explore, revealing the map.
3. Encounter enemies whom the player fights or flees from.
4. Find power-ups and use them to strengthen the player.
5. Locate the exit to the next level - go to 1.

### Minimum Viable Product
- [x] Create a basic dungeon map.
- [x] Place the player and let them walk around.
- [ ] Spawn monsters, draw them, and let the player kill them by walking into them.
- [ ] Add health and a combat system that uses it.
- [ ] Add healing potions.
- [ ] Display a "game over" screen when the player dies.
- [ ] Add the Amulet of Yala to the level and let the player win by reaching it.

### Stretch Goals
- [ ] Add Fields-of-View.
- [ ] Add more interesting dungeon designs.
- [ ] Add some dungeon themes.
- [ ] Ad multiple layers to the dungeon, the the Amulet on the last one.
- [ ] Add varied weapons to the game.
- [ ] Move to a data-driven design for spawning enemies.
- [ ] Consider some visual effects to make the combat more visceral.
- [ ] Consider keeping score.

