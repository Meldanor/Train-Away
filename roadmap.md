1. Level component
    - should load a json file which defines the level
    - entries consists of the folowing tuple:
        - hex coordinates (use the crate https://github.com/ManevilleF/hexx for the hexx handling)
        * type of tile:
            - train + orientation
            - track + orientation
            - start
            - end
    - visualize the level in a crude way
    - Reset level functionality
    - basic 2D pixel art
2. Train component
    - Train should move on a click without an obstacle, but have to follow the tracks
    - Train loose speed in curves
    - Handle intersections (maybe later)
    - basic 2D pixel art
3. Start and goal detection
    - End the game when Sam reaches the goal
    - Motivation: Have at least a simple "game" by this step
4. Train collisions
    - Detect when two trains collide and handle speed
    - Train boost on hit and stop on frontal collision
    - Partial boost on diagonal hits
    - Questions what to do with train intersecting each other at differnet speeds? Maybe no round based calculation but a contininous check? The start delay is then controlled in seconds / power?
    - Trains have differnt color tins based on their number in the game.
      -> Accessibility? Add the number as a text to the train?
5. Train planing and execution
    - Create simple level of three trains that need to be activated in a certain order to reach the goal
        - First train
    - Clicking on a train in planning phase increase the round delay when its start
    - Player can start the execution and watch the chaos unfold
6. Points
    - Measure the time for reaching the goal
    - Create point system for the collision increasement
7. Add sounds and animations
    - Add Train ChooChoo Sounds
        - Speedup Sound depending on velocity?
    - Basic background music -> 1BitDragon or Bosca
    - Add Train Collision sounds
    - "Train in curve"-screetch sound?
    - Start sound?
    - Goal celebration sound
    - Train sprite animation
    - Train Collision SFX "boom" graphic?
8. Create some levels
    - Create some levels make them startable from the menu (no autoprogression at the moment)
9. Credits and main menu overhaul
10. Improve graphics / animations / sounds
    - Multiple train sprites
    - Better backgrounds
    - Improve the sound
11. Live view of current points instead of the end?
