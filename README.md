# Rust Physics Simulator

## How to run:
1. `git clone https://github.com/Dannyhvv/rust-physics-sim`
2. `cd rust-physics-sim`
4. `cargo run`

Switch between tools with tabs on top left

Tools:
- Drag
 - Click and hold left mouse to pull nearest shape towards cursor
-Rect
 -Click and drag to outline a rectangle, release to confirm. 
-Circle
 -Click and drag to outline a circle, release to confirm.

 Physics bodies usually have the following properties, configurable via the UI:
 -Density
  -Greater values make the object "heavier", more resistant to change.
-Restitution
 -Greater values make the object bouncier, conserving more velocity after a collision.
-Static
 -If checked, the object will be unable to move.
-Collide
 -If checked, the object will be able to collide with other bodies.
-Random Colors
 -If checked, the object will recieve a random color from a preset list upon creation, otherwise it will be white. (custom colors planned)
 
World options:
-Paused
 -If checked, time will not progress. Shapes can still be created.
-Gravity X
 -Determines the gravity on the X axis, negative values pull objects towards the left, positive values pull objects towards the right.
-Gravity Y
 -Determines the gravity on the Y axis, negative values pull objects towards the top, positive values pull objects towards the bottom.
-Iterations
 -How many physics checks to do per frame, higher leads to more accurate collisions with worse performance, while lower has better performance with less accurate collisions.
